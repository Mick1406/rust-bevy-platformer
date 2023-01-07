use bevy::prelude::* ;
use bevy_asset_loader::prelude::* ;

fn main() {
    let mut app = App::new() ;
    LoadingState::new(GameState::AssetLoading)
        .continue_to_state(GameState::Playing)
        .with_collection::<ImageAssets>()
        .build(&mut app) ;
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run() ;
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "sandbox.ldtk")]
    map: Handle<LdtkAsset>,
}

