use bevy::{
    asset::{AssetId, uuid::Uuid},
    shader::{
        Shader, ShaderCache, ShaderCacheError, ShaderCacheSource, ShaderDefVal, ValidateShader,
    },
};

fn load_shader(
    _: &(),
    source: ShaderCacheSource,
    _: &ValidateShader,
) -> Result<String, ShaderCacheError> {
    match source {
        ShaderCacheSource::Wgsl(source) => Ok(source),
        _ => unreachable!("WESL shaders should compile to WGSL"),
    }
}

#[test]
fn bar2d_wesl_compiles_with_both_conditional_branches() {
    let mut cache = ShaderCache::new((), load_shader);
    let view_bindings = Shader::from_wesl(
        r#"
struct View {
    clip_from_world: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> view: View;
"#,
        "embedded://bevy_sprite_render/mesh2d/view_bindings.wesl",
    );
    let functions = Shader::from_wesl(
        r#"
fn get_world_from_local(instance_index: u32) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );
}
"#,
        "embedded://bevy_sprite_render/mesh2d/functions.wesl",
    );
    let shader = Shader::from_wesl(
        include_str!("../assets/shaders/bar2d.wesl"),
        "embedded://bevy_health_bar3d/assets/shaders/bar2d.wesl",
    );

    cache.set_shader(AssetId::Uuid { uuid: Uuid::from_u128(1) }, view_bindings);
    cache.set_shader(AssetId::Uuid { uuid: Uuid::from_u128(2) }, functions);
    let shader_id = AssetId::Uuid { uuid: Uuid::from_u128(3) };
    cache.set_shader(shader_id, shader);

    for mut shader_defs in [
        vec![
            ShaderDefVal::from("HAS_BORDER"),
            ShaderDefVal::from("IS_VERTICAL"),
        ],
        vec![ShaderDefVal::from("HAS_BORDER")],
        vec![ShaderDefVal::from("IS_VERTICAL")],
        vec![],
    ] {
        shader_defs.push(ShaderDefVal::UInt("MATERIAL_BIND_GROUP".into(), 2));
        cache.get(0, shader_id, &shader_defs).unwrap();
    }
}
