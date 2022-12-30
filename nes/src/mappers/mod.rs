use std::{cell::RefCell, rc::Rc};

use mos6502::memory::Bus;

use crate::cartridge::{Cartridge, Mirroring};

mod mmc1;
mod nrom;
mod mapper3;

pub trait Mapper : Bus {
  fn mirroring(&self) -> Mirroring;
}

pub(crate) fn for_cart(cart: Cartridge) -> Rc<RefCell<dyn Mapper>> {
  match cart.mapper_type() {
    crate::cartridge::MapperType::Nrom => Rc::new(RefCell::new(nrom::NROM::new(cart))),
    crate::cartridge::MapperType::Mmc1 => Rc::new(RefCell::new(mmc1::MMC1::new(cart))),
    crate::cartridge::MapperType::Mapper3 => Rc::new(RefCell::new(mapper3::Mapper3::new(cart))),
  }
}