use amethyst::ecs::{System,ReadStorage,WriteStorage,Join};
use amethyst::core::Transform;
use amethyst::renderer::Camera;
use data::Player;

pub struct CameraFollowPlayerSystem;

impl<'a> System<'a> for CameraFollowPlayerSystem {
    type SystemData = (ReadStorage<'a,Player>,ReadStorage<'a,Camera>,WriteStorage<'a,Transform>);
    fn run(&mut self, (players,cameras,mut transforms): Self::SystemData){
        let mut position_x = 0.0;
        let mut hp = 0.0;
        for (transform,player) in (&transforms,&players).join() {
            position_x = transform.translation.x;
            hp = player.health as f32;
        }

        for (mut transform,_) in (&mut transforms, &cameras).join() {
            // full hp = player 2*3 right
            // 0 hp = player totally to the left (16/9) ratio

            // assumes max_hp = 10
            let target_player_pos_abs = (10.0 / hp) * (2.0/3.0);

            // normal ortho camera goes from [0,1] on x axis
            transform.translation.x = position_x + 0.5 - target_player_pos_abs;
        }
    }
}