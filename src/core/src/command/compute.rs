// Copyright 2017 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use Backend;
use queue::capability::{Compute, Supports};
use super::{CommandBuffer, RawCommandBuffer};


impl<'a, B: Backend, C: Supports<Compute>> CommandBuffer<'a, B, C> {
    ///
    pub fn bind_compute_pipeline(&mut self, pipeline: &B::ComputePipeline) {
        self.raw.bind_compute_pipeline(pipeline)
    }

    ///
    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        self.raw.dispatch(x, y, z)
    }

    ///
    pub fn dispatch_indirect(&mut self, buffer: &B::Buffer, offset: u64) {
        self.raw.dispatch_indirect(buffer, offset)
    }
}
