/*
 * Copyright (C) 2021 The Aero Project Developers.
 *
 * This file is part of The Aero Project.
 *
 * Aero is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Aero is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Aero. If not, see <https://www.gnu.org/licenses/>.
 */

use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;

use crate::fs;
use crate::fs::devfs;
use crate::fs::inode;

use crate::fs::inode::INodeInterface;
use crate::userland::scheduler;
use crate::userland::task::Task;
use crate::utils::Mutex;

use super::keyboard::KeyCode;
use super::keyboard::KeyboardListener;

lazy_static::lazy_static! {
    static ref TTY: Arc<Tty> = Tty::new();
}

const PLAIN_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf0ec, 0xf07f, 0xf009, 0xfb71, 0xfb77, 0xfb65, 0xfb72, 0xfb74, 0xfb79, 0xfb75, 0xfb69,
    0xfb6f, 0xfb70, 0xf0e8, 0xf02b, 0xf201, 0xf702, 0xfb61, 0xfb73, 0xfb64, 0xfb66, 0xfb67, 0xfb68,
    0xfb6a, 0xfb6b, 0xfb6c, 0xf0f2, 0xf0e0, 0xf05c, 0xf700, 0xf0f9, 0xfb7a, 0xfb78, 0xfb63, 0xfb76,
    0xfb62, 0xfb6e, 0xfb6d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf100,
    0xf101, 0xf102, 0xf103, 0xf104, 0xf105, 0xf106, 0xf107, 0xf108, 0xf109, 0xf208, 0xf209, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf10a, 0xf10b, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const SHIFT_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf021, 0xf022, 0xf0a3, 0xf024, 0xf025, 0xf026, 0xf02f, 0xf028, 0xf029, 0xf03d,
    0xf03f, 0xf05e, 0xf07f, 0xf009, 0xfb51, 0xfb57, 0xfb45, 0xfb52, 0xfb54, 0xfb59, 0xfb55, 0xfb49,
    0xfb4f, 0xfb50, 0xf0e9, 0xf02a, 0xf201, 0xf702, 0xfb41, 0xfb53, 0xfb44, 0xfb46, 0xfb47, 0xfb48,
    0xfb4a, 0xfb4b, 0xfb4c, 0xf0e7, 0xf0b0, 0xf07c, 0xf700, 0xf0a7, 0xfb5a, 0xfb58, 0xfb43, 0xfb56,
    0xfb42, 0xfb4e, 0xfb4d, 0xf03b, 0xf03a, 0xf05f, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf10c,
    0xf10d, 0xf10e, 0xf10f, 0xf110, 0xf111, 0xf112, 0xf113, 0xf11e, 0xf11f, 0xf208, 0xf203, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03e, 0xf120, 0xf121, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf20b, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf20a, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const ALTGR_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf07b, 0xf07d, 0xf07e,
    0xf060, 0xf0ed, 0xf07f, 0xf009, 0xfb71, 0xfb77, 0xf0a4, 0xfb72, 0xfb74, 0xfb79, 0xfb75, 0xfb69,
    0xfb6f, 0xfb70, 0xf05b, 0xf05d, 0xf201, 0xf702, 0xfb61, 0xfb73, 0xfb64, 0xfb66, 0xfb67, 0xfb68,
    0xfb6a, 0xfb6b, 0xfb6c, 0xf040, 0xf023, 0xf05c, 0xf700, 0xf0fa, 0xfb7a, 0xfb78, 0xf0a2, 0xfb76,
    0xfb62, 0xfb6e, 0xfb6d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const SHIFT_ALTGR_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf0ec, 0xf07f, 0xf009, 0xfb51, 0xfb57, 0xfb45, 0xfb52, 0xfb54, 0xfb59, 0xfb55, 0xfb49,
    0xfb4f, 0xfb50, 0xf0e8, 0xf02b, 0xf201, 0xf702, 0xfb41, 0xfb53, 0xfb44, 0xfb46, 0xfb47, 0xfb48,
    0xfb4a, 0xfb4b, 0xfb4c, 0xf0f2, 0xf0e0, 0xf05c, 0xf700, 0xf0f9, 0xfb5a, 0xfb58, 0xf0a9, 0xfb56,
    0xfb42, 0xfb4e, 0xfb4d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const CTRL_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf0ec, 0xf07f, 0xf009, 0xf011, 0xf017, 0xf005, 0xf012, 0xf014, 0xf019, 0xf015, 0xf009,
    0xf00f, 0xf010, 0xf0e8, 0xf02b, 0xf201, 0xf702, 0xf001, 0xf013, 0xf004, 0xf006, 0xf007, 0xf008,
    0xf00a, 0xf00b, 0xf00c, 0xf0f2, 0xf0e0, 0xf01c, 0xf700, 0xf0f9, 0xf01a, 0xf018, 0xf003, 0xf016,
    0xf002, 0xf00e, 0xf00d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xfa02, 0xf122,
    0xf123, 0xf124, 0xf125, 0xf126, 0xf127, 0xf128, 0xf129, 0xf12a, 0xf12b, 0xf208, 0xf204, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf12c, 0xf12d, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const SHIFT_CTRL_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf01e, 0xf07f, 0xf009, 0xf011, 0xf017, 0xf005, 0xf012, 0xf014, 0xf019, 0xf015, 0xf009,
    0xf00f, 0xf010, 0xf0e8, 0xf02b, 0xf201, 0xf702, 0xf001, 0xf013, 0xf004, 0xf006, 0xf007, 0xf008,
    0xf00a, 0xf00b, 0xf00c, 0xf0f2, 0xf0e0, 0xf05c, 0xf700, 0xf0f9, 0xf01a, 0xf018, 0xf003, 0xf016,
    0xf002, 0xf00e, 0xf00d, 0xf02c, 0xf02e, 0xf01f, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const ALTGR_CTRL_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf0ec, 0xf07f, 0xf009, 0xf011, 0xf017, 0xf005, 0xf012, 0xf014, 0xf019, 0xf015, 0xf009,
    0xf00f, 0xf010, 0xf0e8, 0xf01d, 0xf201, 0xf702, 0xf001, 0xf013, 0xf004, 0xf006, 0xf007, 0xf008,
    0xf00a, 0xf00b, 0xf00c, 0xf0f2, 0xf0e0, 0xf05c, 0xf700, 0xf0f9, 0xf01a, 0xf018, 0xf003, 0xf016,
    0xf002, 0xf00e, 0xf00d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const ALT_MAP: &[u16; 256] = &[
    0xf200, 0xf81b, 0xf831, 0xf832, 0xf833, 0xf834, 0xf835, 0xf836, 0xf837, 0xf838, 0xf839, 0xf830,
    0xf827, 0xf8ec, 0xf87f, 0xf809, 0xf871, 0xf877, 0xf865, 0xf872, 0xf874, 0xf879, 0xf875, 0xf869,
    0xf86f, 0xf870, 0xf0e8, 0xf82b, 0xf201, 0xf702, 0xf861, 0xf873, 0xf864, 0xf866, 0xf867, 0xf868,
    0xf86a, 0xf86b, 0xf86c, 0xf0f2, 0xf0e0, 0xf85c, 0xf700, 0xf0f9, 0xf87a, 0xf878, 0xf863, 0xf876,
    0xf862, 0xf86e, 0xf86d, 0xf82c, 0xf82e, 0xf82d, 0xf700, 0xf30c, 0xf703, 0xf820, 0xf207, 0xf500,
    0xf501, 0xf502, 0xf503, 0xf504, 0xf505, 0xf506, 0xf507, 0xf508, 0xf509, 0xf208, 0xf202, 0xf907,
    0xf908, 0xf909, 0xf30b, 0xf904, 0xf905, 0xf906, 0xf30a, 0xf901, 0xf902, 0xf903, 0xf900, 0xf310,
    0xf206, 0xf200, 0xf83c, 0xf50a, 0xf50b, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf212, 0xf118, 0xf210, 0xf211, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const SHIFT_ALT_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf821, 0xf822, 0xf033, 0xf824, 0xf825, 0xf826, 0xf82f, 0xf828, 0xf829, 0xf83d,
    0xf83f, 0xf85e, 0xf07f, 0xf009, 0xf851, 0xf857, 0xf845, 0xf852, 0xf854, 0xf859, 0xf855, 0xf849,
    0xf84f, 0xf850, 0xf0e8, 0xf82a, 0xf201, 0xf702, 0xf841, 0xf853, 0xf844, 0xf846, 0xf847, 0xf848,
    0xf84a, 0xf84b, 0xf84c, 0xf0f2, 0xf0e0, 0xf87c, 0xf700, 0xf0f9, 0xf85a, 0xf858, 0xf843, 0xf856,
    0xf842, 0xf84e, 0xf84d, 0xf83b, 0xf83a, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf83e, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const ALTGR_ALT_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf87b, 0xf87d, 0xf87e,
    0xf860, 0xf0ec, 0xf07f, 0xf009, 0xf871, 0xf877, 0xf865, 0xf872, 0xf874, 0xf879, 0xf875, 0xf869,
    0xf86f, 0xf870, 0xf85b, 0xf85d, 0xf201, 0xf702, 0xf861, 0xf873, 0xf864, 0xf866, 0xf867, 0xf868,
    0xf86a, 0xf86b, 0xf86c, 0xf840, 0xf823, 0xf05c, 0xf700, 0xf0f9, 0xf87a, 0xf878, 0xf863, 0xf876,
    0xf862, 0xf86e, 0xf86d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const CTRL_ALT_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf0ec, 0xf07f, 0xf009, 0xf811, 0xf817, 0xf805, 0xf812, 0xf814, 0xf819, 0xf815, 0xf809,
    0xf80f, 0xf810, 0xf0e8, 0xf02b, 0xf201, 0xf702, 0xf801, 0xf813, 0xf804, 0xf806, 0xf807, 0xf808,
    0xf80a, 0xf80b, 0xf80c, 0xf0f2, 0xf0e0, 0xf81c, 0xf700, 0xf0f9, 0xf81a, 0xf818, 0xf803, 0xf816,
    0xf802, 0xf80e, 0xf80d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf500,
    0xf501, 0xf502, 0xf503, 0xf504, 0xf505, 0xf506, 0xf507, 0xf508, 0xf509, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf20c,
    0xf206, 0xf200, 0xf03c, 0xf50a, 0xf50b, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf20c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const SHIFT_CTRL_ALT_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf81e, 0xf07f, 0xf009, 0xf811, 0xf817, 0xf805, 0xf812, 0xf814, 0xf819, 0xf815, 0xf809,
    0xf80f, 0xf810, 0xf0e8, 0xf02b, 0xf201, 0xf702, 0xf801, 0xf813, 0xf804, 0xf806, 0xf807, 0xf808,
    0xf80a, 0xf80b, 0xf80c, 0xf0f2, 0xf0e0, 0xf05c, 0xf700, 0xf0f9, 0xf81a, 0xf818, 0xf803, 0xf816,
    0xf802, 0xf80e, 0xf80d, 0xf02c, 0xf02e, 0xf81f, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

const ALTGR_CTRL_ALT_MAP: &[u16; 256] = &[
    0xf200, 0xf01b, 0xf031, 0xf032, 0xf033, 0xf034, 0xf035, 0xf036, 0xf037, 0xf038, 0xf039, 0xf030,
    0xf027, 0xf0ec, 0xf07f, 0xf009, 0xf811, 0xf817, 0xf805, 0xf812, 0xf814, 0xf819, 0xf815, 0xf809,
    0xf80f, 0xf810, 0xf0e8, 0xf81d, 0xf201, 0xf702, 0xf801, 0xf813, 0xf804, 0xf806, 0xf807, 0xf808,
    0xf80a, 0xf80b, 0xf80c, 0xf0f2, 0xf0e0, 0xf05c, 0xf700, 0xf0f9, 0xf81a, 0xf818, 0xf803, 0xf816,
    0xf802, 0xf80e, 0xf80d, 0xf02c, 0xf02e, 0xf02d, 0xf700, 0xf30c, 0xf703, 0xf020, 0xf207, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf208, 0xf200, 0xf307,
    0xf308, 0xf309, 0xf30b, 0xf304, 0xf305, 0xf306, 0xf30a, 0xf301, 0xf302, 0xf303, 0xf300, 0xf310,
    0xf206, 0xf200, 0xf03c, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf30e, 0xf702, 0xf30d, 0xf20e, 0xf701, 0xf205, 0xf114, 0xf603, 0xf118, 0xf601, 0xf602, 0xf117,
    0xf600, 0xf119, 0xf115, 0xf116, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf11d,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200, 0xf200,
    0xf200, 0xf200, 0xf200, 0xf200,
];

struct StdinBuffer {
    back_buffer: Vec<u8>,
    front_buffer: Vec<u8>,
    i: usize,
}

impl StdinBuffer {
    #[inline]
    fn new() -> Self {
        Self {
            back_buffer: Vec::new(),
            front_buffer: Vec::new(),
            i: 0,
        }
    }

    #[inline]
    fn swap_buffer(&mut self) {
        self.front_buffer
            .resize(self.back_buffer.len() - self.i, 0x00);
        self.front_buffer
            .copy_from_slice(&self.back_buffer[self.i..]);
        self.i = self.back_buffer.len() - 1;
    }
}

struct TtyState {
    lshift: bool,
    rshift: bool,
    lctrl: bool,
    rctrl: bool,
    lalt: bool,
    altgr: bool,
    caps: bool,
}

struct Tty {
    device_id: usize,
    state: Mutex<TtyState>,
    sref: Weak<Self>,

    stdin: Mutex<StdinBuffer>,
    task: Mutex<Arc<Task>>,
}

impl Tty {
    #[inline]
    fn new() -> Arc<Self> {
        Arc::new_cyclic(|sref| Self {
            device_id: devfs::alloc_device_marker(),
            state: Mutex::new(TtyState {
                lshift: false,
                rshift: false,
                lctrl: false,
                rctrl: false,
                lalt: false,
                altgr: false,
                caps: false,
            }),
            task: Mutex::new(scheduler::get_scheduler().current_task()),
            stdin: Mutex::new(StdinBuffer::new()),
            sref: sref.clone(),
        })
    }
}

impl INodeInterface for Tty {
    #[inline]
    fn read_at(&self, _offset: usize, buffer: &mut [u8]) -> fs::Result<usize> {
        *self.task.lock_irq() = scheduler::get_scheduler().current_task();
        scheduler::get_scheduler().inner.await_io();

        let mut stdin = self.stdin.lock_irq();

        stdin.front_buffer.resize(buffer.len(), 0x00);
        buffer.copy_from_slice(&stdin.front_buffer);
        Ok(buffer.len())
    }

    #[inline]
    fn write_at(&self, _offset: usize, buffer: &[u8]) -> fs::Result<usize> {
        unsafe {
            crate::prelude::print!("{}", core::str::from_utf8_unchecked(buffer));
            Ok(buffer.len())
        }
    }
}

impl devfs::Device for Tty {
    #[inline]
    fn device_marker(&self) -> usize {
        self.device_id
    }

    #[inline]
    fn device_name(&self) -> &str {
        "tty"
    }

    #[inline]
    fn inode(&self) -> Arc<dyn inode::INodeInterface> {
        self.sref.upgrade().unwrap()
    }
}

impl KeyboardListener for Tty {
    fn on_key(&self, key: KeyCode, released: bool) {
        let mut state = self.state.lock();

        match key {
            KeyCode::KEY_CAPSLOCK if !released => state.caps = !state.caps,
            KeyCode::KEY_ENTER | KeyCode::KEY_KPENTER if !released => {
                let mut stdin = self.stdin.lock_irq();
                stdin.back_buffer.push('\n' as u8);
                stdin.swap_buffer();

                crate::prelude::print!("\n");

                let task = self.task.lock_irq();
                scheduler::get_scheduler().inner.wake_up(task.clone());
            }

            KeyCode::KEY_BACKSPACE if !released => {}

            KeyCode::KEY_LEFTSHIFT => state.lshift = !released,
            KeyCode::KEY_RIGHTSHIFT => state.rshift = !released,

            KeyCode::KEY_LEFTCTRL => state.lctrl = !released,
            KeyCode::KEY_RIGHTCTRL => state.rctrl = !released,

            KeyCode::KEY_LEFTALT => state.lalt = !released,
            KeyCode::KEY_RIGHTALT => state.altgr = !released,

            _ if !released => {
                let mut shift = state.lshift || state.rshift;
                let ctrl = state.lctrl || state.rctrl;

                if state.caps && state.caps {
                    shift = !shift;
                }

                let map = match (shift, ctrl, state.lalt, state.altgr) {
                    (false, false, false, false) => PLAIN_MAP,
                    (true, false, false, false) => SHIFT_MAP,
                    (false, true, false, false) => CTRL_MAP,
                    (false, false, true, false) => ALT_MAP,
                    (false, false, false, true) => ALTGR_MAP,
                    (true, true, false, false) => SHIFT_CTRL_MAP,
                    (false, true, true, false) => CTRL_ALT_MAP,
                    (false, true, true, true) => ALTGR_CTRL_ALT_MAP,
                    (true, true, true, false) => SHIFT_CTRL_ALT_MAP,
                    (false, false, true, true) => ALTGR_ALT_MAP,
                    (true, false, true, false) => SHIFT_ALT_MAP,
                    (false, true, false, true) => ALTGR_CTRL_MAP,
                    (true, false, false, true) => SHIFT_ALTGR_MAP,
                    _ => PLAIN_MAP,
                };

                let character =
                    unsafe { core::char::from_u32_unchecked((map[key as usize] & 0xff) as _) };

                self.stdin.lock_irq().back_buffer.push(character as u8);
                crate::prelude::print!("{}", character);
            }

            _ => {}
        }
    }
}

fn init_tty() {
    super::keyboard::register_keyboard_listener(TTY.as_ref().clone());

    devfs::install_device(TTY.clone()).expect("failed to register tty as a device");
}

crate::module_init!(init_tty);
