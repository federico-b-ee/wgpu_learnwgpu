/* 
fn set_render_pipeline_vertex (&mut self, buffer: VertexBufferLayout) {
    let render_pipeline_layout =
        self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

    let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &self.shader,
            entry_point: "vs_main", 
            buffers: &[buffer,],           
        },
        fragment: Some(wgpu::FragmentState {
            module: &self.shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: self.config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, 
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None, 
        multisample: wgpu::MultisampleState {
            count: 1,                        
            mask: !0,                       
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    self.render_pipeline = render_pipeline;
    }  
    fn set_render_pipeline (&mut self) {
    let render_pipeline_layout =
        self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

    let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &self.shader,
            entry_point: "vs_main", 
            buffers: &[],           
        },
        fragment: Some(wgpu::FragmentState {
            module: &self.shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: self.config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, 
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None, 
        multisample: wgpu::MultisampleState {
            count: 1,                        
            mask: !0,                       
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    self.render_pipeline = render_pipeline;
    }  


    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });




fn input
  match event {
            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase: _,
                modifiers: _,
            } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    if *y > 0.0 && self.color.b + 0.1 <= 1.0 {
                        self.color.b += 0.1;
                    }
                    if *y < 0.0 && self.color.b - 0.1 >= 0.0 {
                        self.color.b -= 0.1;
                    }
                    if *x > 0.0 && self.color.r + 0.1 <= 1.0 {
                        self.color.r += 0.1;
                    }
                    if *x < 0.0 && self.color.r - 0.1 >= 0.0 {
                        self.color.r -= 0.1;
                    }
                }
                _ => {}
            },

            WindowEvent::CursorMoved {
                device_id: _,
                position,
                modifiers: _,
            } => {
                self.color.r = position.x / self.size.width as f64;
                self.color.g = position.y / self.size.height as f64;
            }

            WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } => {
                if input.state == ElementState::Pressed {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Key1) => {
                            self.debug_material = {
                                let diffuse_bytes = include_bytes!("../res/cube-diffuse.jpg");
                                let normal_bytes = include_bytes!("../res/cube-normal.png");
                    
                                let diffuse_texture = texture::Texture::from_bytes(&self.device, &self.queue, diffuse_bytes, "cube-d", false).unwrap();
                                let normal_texture = texture::Texture::from_bytes(&self.device, &self.queue, normal_bytes, "cube-n", true).unwrap();
                            
                                model::Material::new(&self.device, "alt-material", diffuse_texture, normal_texture, &self.texture_bind_group_layout)
                            };    
                        },
                        Some(VirtualKeyCode::Key2) => {
                            self.debug_material = {
                                let diffuse_bytes = include_bytes!("../res/metal.png");
                                let normal_bytes = include_bytes!("../res/metal-normal.png");
                    
                                let diffuse_texture = texture::Texture::from_bytes(&self.device, &self.queue, diffuse_bytes, "metal-d", false).unwrap();
                                let normal_texture = texture::Texture::from_bytes(&self.device, &self.queue, normal_bytes, "metal-n", true).unwrap();
                            
                                model::Material::new(&self.device, "alt-material", diffuse_texture, normal_texture, &self.texture_bind_group_layout)
                            };    
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        true
*/