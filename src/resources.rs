use amethyst::renderer::{ImageFormat, SpriteRender, SpriteSheet, sprite::SpriteSheetHandle, SpriteSheetFormat, Texture};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::ui::{FontHandle, TtfFormat};
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

    pub fn grid_sprite_render(&self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.handle.clone(),
            sprite_number: 0,
        }
    }

    pub fn shop_sprite_render(&self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.handle.clone(),
            sprite_number: 1,
        }
    }

    pub fn reserve_sprite_render(&self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.handle.clone(),
            sprite_number: 2,
        }
    }

    pub fn character_sprite_render(&self) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.handle.clone(),
            sprite_number: 3,
        }
    }
}

pub struct Font {
}

impl Font {
    pub fn square(world: &World) -> FontHandle {
        world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        )
    }
}
