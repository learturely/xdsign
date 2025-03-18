// Copyright (C) 2025 worksoup <https://github.com/worksoup/>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fs;

use ico::{IconDirEntry, IconImage};
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
fn need_icon() -> bool {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    matches!(target_os.as_str(), "windows")
}
fn handle_resource() {
    println!("cargo:rerun-if-changed={LOGO_RC}");
    println!("cargo:rerun-if-changed={LOGO_SVG}");
    println!("cargo:rerun-if-changed={LOGO_ICO}");
    if !need_icon() {
        return;
    }
    const LOGO_RC: &str = "res/logo.rc";
    const LOGO_ICO: &str = "res/logo.ico";
    const LOGO_SVG: &str = "res/logo.svg";
    const SVG_SIZE: f32 = 64.0;
    if fs::metadata(LOGO_ICO).is_err() {
        const ICO_SIZE: [u32; 6] = [16, 32, 48, 64, 128, 256];
        let svg_data = fs::read(LOGO_SVG).unwrap();
        let tree = Tree::from_data(&svg_data, &Options::default()).unwrap();
        let mut icon = ico::IconDir::new(ico::ResourceType::Icon);
        for size in ICO_SIZE {
            let mut pixmap = Pixmap::new(size, size).unwrap();
            let scale = size as f32 / SVG_SIZE;
            resvg::render(
                &tree,
                Transform::from_row(scale, 0.0, 0.0, scale, 0.0, 0.0),
                &mut pixmap.as_mut(),
            );
            // pixmap
            //     .save_png(format!("res/logo_{0}x{0}.png", size))
            //     .unwrap();
            let png_data = pixmap.encode_png().unwrap();
            let image = IconImage::read_png(&*png_data).unwrap();
            icon.add_entry(IconDirEntry::encode_as_png(&image).unwrap());
        }
        let ico_file = fs::File::create(LOGO_ICO).unwrap();
        icon.write(ico_file).unwrap();
    }
    embed_resource::compile(LOGO_RC, embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
fn main() {
    handle_resource();
}
