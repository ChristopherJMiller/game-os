use core::{cell::RefCell};

use alloc::vec::Vec;
use bare_metal::Mutex;

use crate::{cpu::free, mem::mailbox_heap_location};

/// Mailbox Inner Components
pub struct MailBoxInner;

/// Mailbox Framebuffer
/// https://elinux.org/RPi_Framebuffer
pub struct MailBox(Mutex<RefCell<MailBoxInner>>);

impl MailBox {
  pub const fn new() -> Self {
    Self(Mutex::new(RefCell::new(MailBoxInner {})))
  }
}

#[derive(Copy, Clone)]
pub enum MailboxChannel {
  Property,
}

impl Into<u32> for MailboxChannel {
  fn into(self) -> u32 {
    match self {
      MailboxChannel::Property => 8,
    }
  }
}

const MAILBOX_BASE: u32 = 0x3F00B880;
const MAILBOX_READ: u32 = MAILBOX_BASE + 0x0;
const MAILBOX_STATUS: u32 = MAILBOX_BASE + 0x18;
const MAILBOX_WRITE: u32 = MAILBOX_BASE + 0x20;


/// Global instance of Mailbox Framebuffer
const MAILBOX: MailBox = MailBox::new();

impl MailBoxInner {
  // https://jsandler18.github.io/extra/mailbox.html
  pub fn read(&self, channel: MailboxChannel) -> u32 {
    loop {
      // Wait until not empty
      loop {
        //Read MAIL0_STATUS
        let status = unsafe { core::ptr::read_volatile(MAILBOX_STATUS as *mut u32) };
        if !MailStatus::from(status).empty {
          break;
        }
      }

      // Read MAIL0_READ
      let message = unsafe { core::ptr::read_volatile(MAILBOX_READ as *mut u32) };
      let mail_message = MailMessage::from(message);
      if mail_message.channel as u32 == channel.into() {
        return message;
      }
    }
  }

  pub fn send(&self, channel: MailboxChannel, data: &mut [u32]) {
    // Wait until not full
    loop {
      //Read MAIL0_STATUS
      let status = unsafe { core::ptr::read_volatile(MAILBOX_STATUS as *mut u32) };
      if !MailStatus::from(status).full {
        break;
      }
    }

    // Send
    // Write to MAIL0_WRITE
    let channel: u32 = channel.into();
    // Write to structure location (1 << 22)
    unsafe { core::ptr::copy(data.as_ptr(), mailbox_heap_location() as *mut u32, data.len()) };

    let data = mailbox_heap_location() as u32 | channel;

    unsafe { core::ptr::write_volatile(MAILBOX_WRITE as *mut u32, data) };
  }
}

/// Copies back data from structure location (1 << 22) to variable.
/// Assumes you know that it's the correct size.
pub unsafe fn update_property_message(data: &mut [u32]) {
  core::ptr::copy(mailbox_heap_location() as *mut u32, data.as_mut_ptr(), data.len());
}

pub struct MailMessage {
  pub channel: u8,
  pub data: u32,
}

impl From<u32> for MailMessage {
  fn from(message: u32) -> Self {
    Self {
      channel: (message & 0xF) as u8,
      data: (message & 0xFFFF_FFF0) >> 4
    }
  }
}

#[derive(Debug)]
pub struct MailStatus {
  pub empty: bool,
  pub full: bool
}

impl From<u32> for MailStatus {
  fn from(status: u32) -> Self {
    Self {
      empty: (status & (1 << 30)) != 0,
      full: (status & (1 << 31)) != 0,
    }
  }
}

pub enum PropertyMessage {
  AllocateBuffer(u32),
  ReleaseBuffer,
  GetPhysicalDimensions,
  SetPhysicalDimensions(u32, u32),
  GetVirtualDimensions,
  SetVirtualDimensions(u32, u32),
  GetBitsPerPixel,
  SetBitsPerPixel(u32),
  GetBytesPerRow,
}

impl PropertyMessage {
  pub fn to_buffer(&self) -> Vec<u32> {
    match self {
      PropertyMessage::AllocateBuffer(s) => [self.into(), 8, 0, *s, 0].into(),
      PropertyMessage::GetPhysicalDimensions => [self.into(), 8, 0, 0, 0].into(),
      PropertyMessage::SetPhysicalDimensions(x, y) => [self.into(), 8, 0, *x, *y].into(),
      PropertyMessage::GetVirtualDimensions => [self.into(), 8, 0, 0, 0].into(),
      PropertyMessage::SetVirtualDimensions(x, y) => [self.into(), 8, 0, *x, *y].into(),
      PropertyMessage::GetBitsPerPixel => [self.into(), 4, 0, 0].into(),
      PropertyMessage::GetBytesPerRow => [self.into(), 4, 0, 0].into(),
      PropertyMessage::SetBitsPerPixel(x) => [self.into(), 4, 0, *x].into(),
      _ => [self.into(), 0, 0].into(),
    }
  }

  pub const fn get_end_buffer() -> [u32; 1] {
    [0; 1]
  }
}

impl Into<u32> for &PropertyMessage {
  fn into(self) -> u32 {
    match self {
      PropertyMessage::AllocateBuffer(_) => 0x00040001,
      PropertyMessage::ReleaseBuffer => 0x00048001,
      PropertyMessage::GetPhysicalDimensions => 0x00040003,
      PropertyMessage::SetPhysicalDimensions(_, _) => 0x00048003,
      PropertyMessage::GetVirtualDimensions => 0x00040004,
      PropertyMessage::SetVirtualDimensions(_, _) => 0x00048004,
      PropertyMessage::GetBitsPerPixel => 0x00040005,
      PropertyMessage::SetBitsPerPixel(_) => 0x00048005,
      PropertyMessage::GetBytesPerRow => 0x00040008,
    }
  }
}

pub enum BufferRequestResultCode {
  Request,
  ResponseSuccess,
  ResponseError
}

impl Into<u32> for BufferRequestResultCode {
  fn into(self) -> u32 {
    match self {
      BufferRequestResultCode::Request => 0x0,
      BufferRequestResultCode::ResponseSuccess => 0x80000000,
      BufferRequestResultCode::ResponseError => 0x80000001,
    }
  }
}

impl BufferRequestResultCode {
  pub fn from_buffer_data(data: &[u32]) -> Option<Self> {
    match data[1] {
      0x0 => Some(Self::Request),
      0x80000000 => Some(Self::ResponseSuccess),
      0x80000001 => Some(Self::ResponseError),
      _ => None,
    }
  }
}

pub fn build_property_message_buffer(properties: &[PropertyMessage]) -> Vec<u32> {
  let tag_buffers = properties.iter().fold(Vec::<u32>::new(), |acc, x| [acc, x.to_buffer()].concat());
  let mut all_tags = [tag_buffers, PropertyMessage::get_end_buffer().to_vec()].concat();
  // 16 byte align
  let additional_padding = (16 - (((all_tags.len() + 2) * 4) % 16)) >> 2;
  for _ in 0..additional_padding {
    all_tags.push(0);
  }
  let length = (all_tags.len() + 2) * 4;
  let prepend = [length as u32, BufferRequestResultCode::Request.into()];
  [prepend.to_vec(), all_tags].concat()
}

pub fn send_property_messages(properties: &[PropertyMessage]) -> Result<Vec<u32>, &str> {
  let mut buffer = build_property_message_buffer(properties);
  let (_, buffer, _) = unsafe { buffer.align_to_mut::<u32>() };
  free(|cs| {
    if let Ok(lock) = MAILBOX.0.borrow(*cs).try_borrow_mut() {
      lock.send(MailboxChannel::Property, buffer);
      // Wait for Response
      lock.read(MailboxChannel::Property);
      unsafe { update_property_message(buffer); }
    } else {
      panic!("Failed to send property to mailbox");
    }
  });

  if let Some(response) = BufferRequestResultCode::from_buffer_data(buffer) {
    match response {
      BufferRequestResultCode::Request => Err("Got Request Result back! Never processed?"),
      BufferRequestResultCode::ResponseSuccess => Ok(Vec::from(buffer)),
      BufferRequestResultCode::ResponseError => Err("Mailbox returned error!"),
    }
  } else {
    panic!("Unknown Response from Mailbox! {:#?}", buffer[1]);
  }
}
