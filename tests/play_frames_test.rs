use tapciify::play_frames;

#[test]
fn plays_frames() {
    play_frames(
        vec!["./assets/logo.png".to_string()],
        128,
        " .,:;+*?%S#@".to_owned(),
        true,
        None,
        false,
    );
}
