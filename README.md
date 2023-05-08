# os-template

OS Template
This is a template for a simple operating system (OS) built from scratch using Rust programming language. The goal of this project is to provide a basic skeleton that you can build upon to create your own OS or experiment with operating system concepts.

## Features
The OS Template provides a number of features out-of-the-box, including:

- Kernel: A simple kernel that manages memory, interrupts, and basic system calls.
- File System: Support for multiple file systems, including FAT and EXT2.
- Networking: Basic networking support with TCP, UDP, and IP protocols.
- GUI: A simple GUI toolkit with support for basic widgets and layouts.
- Drivers: Support for common drivers, including keyboard, network, and storage.
- Security: Basic security features, including authentication and firewall.
- Testing: A suite of tests for different components of the OS.


[Clone this project](https://github.com/INeddHelp/os-template#installation)

> All the files are in the `src` folder.

![screenshot](https://i.ibb.co/jZ3nWFY/Screenshot-from-2023-05-07-17-31-53.png)

## Tree

.:
arch  boot  core  crypto  db  drivers  fs  gui  kernel  lib  main.rs  mm  net  process  securety  storage  tests  util

./arch:\
armv7.rs  mips.rs  x86_64.rs

./boot:\
bios.rs  grub.rs  uefi.rs

./core:\
config.rs  error.rs  init.rs

./crypto:\
cipher.rs  hash.rs  hmac.rs

./db:\
postgres.rs  redis.rs  sqlite.rs

./drivers:\
gpu.rs  keyboard.rs  network.rs  storage.rs

./fs:\
ext2.rs  fat.rs  nfts.rs  vfs.rs

./gui:\
button.rs  components  event.rs  images  label.rs  layouts  menu.rs  textbox.rs  theme.rs  themes  utils  widget.rs  window.rs

./gui/components:\
button.rs  label.rs  menu.rs  textbox.rs

./gui/images:\
> You have to insert the images

./gui/layouts:\
flex.rs  grid.rs  stack.rs

./gui/themes:\
dark.rs  light.rs

./gui/utils:\
color.rs  font.rs  image.rs  input.rs

./kernel:\
interrupts.rs  memory.rs  scheduler.rs  syscall.rs

./lib:\
collections.rs  io.rs  math.rs  sync.rs

./mm:\
allocator.rs  paging.rs  virtual.rs

./net:\
dns.rs  ip.rs  tcp.rs  udp.rs

./process:\
ipc.rs  process.rs  thread.rs

./securety:\
auth.rs  firewall.rs  tls.rs

./storage:\
block.rs  inode.rs  journal.rs

./tests:\
keyboard_test.rs  network_test.rs  unit_test.rs

./util:\
config.rs  logging.rs  time.rs

# Installation 

To clone this project, open the terminal and type:

```bash
git clone https://github.com/INeddHelp/os-template.git
```

# Contributing  

As a template project, it is not meant to be a complete or fully-functional operating system, but rather a starting point for building your own OS. However, contributions to improve the template, fix bugs, or add new features are always welcome!

To contribute, please follow these steps:

- Fork the repository and create a new branch with a descriptive name.
- Make your changes and ensure they pass all tests.
- Commit your changes with a clear and concise commit message.
- Push your changes to your fork.
- Submit a pull request to the main repository.
We appreciate all contributions, and will review pull requests as quickly as possible. However, please note that we are a small team and may not be able to address every issue immediately. If you would like to discuss a potential contribution before starting work, please open an issue and we'll be happy to chat with you!

# License 

This project is under MIT license.
