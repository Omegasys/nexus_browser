/*
 * Nexus Browser - Servo Page
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::frame::Frame;

pub struct Page {
    id: u64,
    main_frame: Frame,
    child_frames: Vec<Frame>,
    open: bool,
}

impl Page {
    pub fn new(
        page_id: u64,
        frame_id: u64,
    ) -> Self {
        let mut main_frame = Frame::new(frame_id);
        main_frame.initialize();

        Self {
            id: page_id,
            main_frame,
            child_frames: Vec::new(),
            open: true,
        }
    }

    pub fn close(&mut self) {
        for frame in &mut self.child_frames {
            frame.close();
        }

        self.child_frames.clear();

        self.main_frame.close();

        self.open = false;
    }

    pub fn navigate(
        &mut self,
        url: &str,
    ) -> Result<(), String> {
        if !self.open {
            return Err(
                "Page is closed".to_string()
            );
        }

        self.main_frame.navigate(url)
    }

    pub fn reload(&mut self) -> Result<(), String> {
        if !self.open {
            return Err(
                "Page is closed".to_string()
            );
        }

        self.main_frame.reload()
    }

    pub fn add_frame(
        &mut self,
        mut frame: Frame,
    ) -> Result<(), String> {
        if !self.open {
            return Err(
                "Page is closed".to_string()
            );
        }

        frame.initialize();

        self.child_frames.push(frame);

        Ok(())
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn main_frame(&self) -> &Frame {
        &self.main_frame
    }

    pub fn main_frame_mut(&mut self) -> &mut Frame {
        &mut self.main_frame
    }

    pub fn frame_count(&self) -> usize {
        if !self.open {
            return 0;
        }

        1 + self.child_frames.len()
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}
