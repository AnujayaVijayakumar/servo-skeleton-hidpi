use gleam::gl;
use glutin::{ContextBuilder, EventsLoop, GlContext, GlWindow, WindowBuilder};
use glow::Context;
use std::process::Command;
use slint::Image;
use std::path::Path;

use servo::{Servo,ServoBuilder};

pub struct Browser {
    current_url: String,
    servo_instance: Servo,
    texture_id: u32,
    width: i32,
    height: i32,
}

impl Browser {
    pub fn new(width: i32, height: i32) -> Self {
        let servo_instance = Servo::new();

        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
        }

        Self {
            current_url: String::new(),
            servo_instance,
            texture_id,
            width,
            height,
        }
    }

    pub fn load_page(&mut self, url: &str) {
        self.current_url = url.to_string();
        println!("Servo browser is loading: {}", self.current_url);

        self.servo_instance.load_url(&self.current_url);
    }

    pub fn render(&mut self) {
        self.servo_instance.render();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::CopyTexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA,
                0,
                0,
                self.width,
                self.height,
                0,
            );
        }

        println!("[Browser] Captured framebuffer into texture {}", self.texture_id);
    }

    pub fn get_texture_id(&self) -> u32 {
        self.texture_id
    }

    pub fn current_page(&self) -> &str {
        &self.current_url
    }
}
pub fn servo_start(width: i32, height: i32) -> Browser {
    Browser::new(width, height)
}

pub fn render_view_as_image(&mut Browser.texture_id) -> slint::Image {

      slint::Image::load_from_path(Path::new("placeholder.png")).unwrap()
    }
