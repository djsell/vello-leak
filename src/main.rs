#[cfg(all(feature = "v9", not(feature = "v10")))]
use v9 as vello;

#[cfg(all(feature = "v10", not(feature = "v9")))]
use v10 as vello;

use vello::wgpu;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;

// 60 fps for 5 minutes
const FRAMES: u32 = 60 * 60 * 5;
const REPORT_INTERVAL: u32 = FRAMES / 10;

#[global_allocator]
static GLOBAL: alloc_metrics::MetricAlloc<std::alloc::System> =
    alloc_metrics::MetricAlloc::new(std::alloc::System);

fn main() {
    let (device, queue, texture) = init_wgpu();

    let scene = vello::Scene::new();
    let mut renderer = vello::Renderer::new(&device, vello::RendererOptions::default()).unwrap();

    for frame in 0..FRAMES {
        renderer
            .render_to_texture(
                &device,
                &queue,
                &scene,
                &texture,
                &vello::RenderParams {
                    base_color: vello::peniko::Color::BLACK,
                    width: WIDTH,
                    height: HEIGHT,
                    antialiasing_method: vello::AaConfig::Area,
                },
            )
            .unwrap();

        if frame % REPORT_INTERVAL == 0 {
            println!("frame {frame}");
            let metrics = alloc_metrics::global_metrics();
            println!("{metrics:?}");
        }
    }
}

fn init_wgpu() -> (wgpu::Device, wgpu::Queue, wgpu::TextureView) {
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let (device, queue) = futures::executor::block_on(async {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::from_env().unwrap_or_default(),
                ..Default::default()
            })
            .await
            .unwrap();

        adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap()
    });

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING,
        view_formats: &[],
    });

    let texture = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (device, queue, texture)
}
