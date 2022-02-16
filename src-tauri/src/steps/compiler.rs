// Copyright (C) 2022 Guyutongxue
//
// This file is part of vscch4.
//
// vscch4 is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// vscch4 is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with vscch4.  If not, see <http://www.gnu.org/licenses/>.

use serde::{Serialize, Deserialize};

pub mod verparse;

pub mod mingw;
pub mod msvc;

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compiler {
  pub setup: String,
  pub path: String,
  pub version: String,
  package_string: String,
}

impl Compiler {
  pub fn new(setup: &CompilerSetup, path: &str, version_text: &str) -> Compiler {
    let (version, package_string) = (setup.verparser)(version_text);
    Compiler {
      setup: setup.id.to_string(),
      path: path.to_string(),
      version: version.to_string(),
      package_string: package_string.to_string(),
    }
  }
}

pub struct CompilerSetup {
  pub id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub how_to_install: &'static str,

  pub scan: fn() -> Vec<Compiler>,
  pub verify: Option<fn(&str) -> Result<Compiler, &'static str>>,
  pub install: Option<fn() -> bool>,

  pub verparser: verparse::Parser,
}

#[cfg(target_os = "windows")]
pub static ENABLED_SETUPS: &[&CompilerSetup] = &[&mingw::GCC_SETUP, &msvc::SETUP];

#[cfg(target_os = "macos")]
pub static ENABLED_SETUPS: &[&CompilerSetup] = &[];

#[cfg(target_os = "linux")]
pub static ENABLED_SETUPS: &[&CompilerSetup] = &[];

pub fn get_setup(id: &str) -> &CompilerSetup {
  ENABLED_SETUPS.iter().find(|s| s.id == id).unwrap()
}