// arch
use arch::{armv7, mips, x86_64};

// boot
use boot::{bios, grub, uefi};

// core
use core::{config, error, init};

// crypto
use crypto::{cipher, hash, hmac};

// db
use db::{postgres, redis, sqlite};

// drivers
use drivers::{gpu, keyboard, network, storage};

// fs
use fs::{ext2, fat, nfts, vfs};

// gui
use gui::{
    button, components, event, images, label, layouts, menu, textbox, theme, themes, utils, widget,
    window,
};
use gui::components::{button as button_component, label as label_component, menu as menu_component, textbox as textbox_component};

// kernel
use kernel::{interrupts, memory, scheduler, syscall};

// lib
use lib::{collections, io, math, sync};

// mm
use mm::{allocator, paging, virtual};

// net
use net::{dns, ip, tcp, udp};

// process
use process::{ipc, process, thread};

// security
use security::{auth, firewall, tls};

// storage
use storage::{block, inode, journal};

// tests
use tests::{keyboard_test, network_test, unit_test};

// util
use util::{config, logging, time};

fn main() {
    // your code here
}
