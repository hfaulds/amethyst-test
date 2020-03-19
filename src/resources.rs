use amethyst::renderer::{ImageFormat, SpriteRender, SpriteSheet, sprite::SpriteSheetHandle, SpriteSheetFormat, Texture};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;

pub struct Sprites {
    pub handle: SpriteSheetHandle,
}

impl Sprites {
    pub fn initialize(world: &mut World) -> Sprites {
        // Load the texture for our sprites. We'll later need to
        // add a handle to this texture to our `SpriteRender`s, so
        // we need to keep a reference to it.
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "sprites/tilemap_packed.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        // Load the spritesheet definition file, which contains metadata on our
        // spritesheet texture.
        let sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "sprites/tilemap_packed.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            )
        };


        let sprites = Sprites { handle: sheet_handle.clone() };
        world.insert(sprites);
        Sprites { handle: sheet_handle }
    }

    pub fn sprite_render(&self, sprite_number: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.handle.clone(),
            sprite_number,
        }
    }
}
