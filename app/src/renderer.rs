use opengl_rs::GfxProgram;
use kms_rs::Graphic;
use nvg_rs::context::Vertex;

use crate::image_scaler;
// use once_cell::sync::Lazy;

pub fn init(graphic: &mut Graphic<GfxProgram>) {
    let (width, height) = (graphic.get_width() as f32, graphic.get_height() as f32);
    let program = graphic.get_tag_mut();
    
    let image = image::ImageReader::open("resources/images/bg_4.png").unwrap().decode().unwrap();
    let (image_width, image_height) = (image.width() as f32, image.height() as f32);
    // let (x, y, w, h) = ((width - image_width) / 2.0, (height - image_height) / 2.0, image_width, image_height);

    let (su, eu, sv, ev) = image_scaler::stretch_image(width as _, height as _, image_width as _, image_height as _);

    let vertexes = vec![
        Vertex::new(0f32, 0f32, su, sv),
        Vertex::new(0f32 + width, 0f32 + height, eu, ev),
        Vertex::new(0f32, 0f32 + height, su, ev),
        Vertex::new(0f32 + width, 0f32, eu, sv)
    ];

    // let vertexes = vec![
    //     Vertex::new(x, y, 0.0, 0.0),
    //     Vertex::new(x + w, y + h, 1.0, 1.0),
    //     Vertex::new(x, y + h, 0.0, 1.0),
    //     Vertex::new(x + w, y, 1.0, 0.0)
    // ];
    let indices: Vec<u32> = vec![
        0, 1, 2,
        0, 3, 1
    ];
    
    let mut vao = 0u32;
    opengl_rs::gen_vertex_arrays(1, &mut vao);
    opengl_rs::bind_vertex_array(vao);
    
    let vbos = opengl_rs::gen_buffers(2);
    opengl_rs::bind_buffer(opengl_rs::def::BufferTarget::ArrayBuffer, vbos[0]);
    opengl_rs::buffer_data(
        opengl_rs::def::BufferTarget::ArrayBuffer,
        vertexes.as_slice(),
        opengl_rs::def::BufferUsageHint::StaticDraw
    );

    opengl_rs::bind_buffer(opengl_rs::def::BufferTarget::ElementArrayBuffer, vbos[1]);
    opengl_rs::buffer_data(
        opengl_rs::def::BufferTarget::ElementArrayBuffer,
        indices.as_slice(),
        opengl_rs::def::BufferUsageHint::StaticDraw
    );

    let vertex_idx = opengl_rs::get_attrib_location(program.get_id(), "aVertex");
    opengl_rs::enable_vertex_attrib_array(vertex_idx);
    opengl_rs::vertex_attrib_pointer_f32(
        vertex_idx, 
        2, 
        false,
        std::mem::size_of::<Vertex>() as _, 
        0);
    let coord_idx = opengl_rs::get_attrib_location(program.get_id(), "aCoord");
    opengl_rs::enable_vertex_attrib_array(coord_idx);
    opengl_rs::vertex_attrib_pointer_f32(
        coord_idx, 
        2, 
        false,
        std::mem::size_of::<Vertex>() as _, 
        (std::mem::size_of::<f32>() * 2) as _);

   
    
    opengl_rs::uniform_1i(opengl_rs::get_uniform_location(program.get_id(), "uTexture"), 0);
    let texture = opengl_rs::GfxTexture::new(opengl_rs::def::TextureUnit::Texture0, opengl_rs::def::TextureFilter::Linear);
    
    let image_data = image.to_rgba8().into_vec();
    let image_data = opengl_rs::ImageData {
        width: image_width as _,
        height: image_height as _,
        value: image_data
    };
    texture.load(&image_data);
    program.add_texture(texture);
}


// static STARTED_TICK: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());
pub fn update(graphic: &mut Graphic<GfxProgram>) {
    // let started_tick = STARTED_TICK.to_owned();
    // let h = std::time::SystemTime::now()
    //     .duration_since(started_tick)
    //     .unwrap()
    //     .as_millis() as f64
    //     / 10_000f64
    //     % 1f64;
    // let hsv = nvg_rs::color::Color::hsl(h as _, 1.0, 0.35);
    // let (r, g, b, a) = hsv.into();
    // opengl_rs::clear_color(r, g, b, a);

    opengl_rs::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    opengl_rs::bind_vertex_array(1);
    // Enable Alpha
    opengl_rs::enable(opengl_rs::def::EnableCap::Blend);
    opengl_rs::blend_func(opengl_rs::def::BlendingFactor::SrcAlpha, opengl_rs::def::BlendingFactor::OneMinusSrcAlpha);
    
    opengl_rs::uniform_1i(opengl_rs::get_uniform_location(graphic.get_tag().get_id(), "uTexture"), 0);

    opengl_rs::draw_elements::<u32>(opengl_rs::def::BeginMode::Triangles, 6, opengl_rs::def::DrawElementsType::UnsignedInt, None);
}