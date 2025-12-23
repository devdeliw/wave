use wave::Stage;

fn main() {
    let mut stage = Stage::new(8, 6);

    assert_eq!(stage.width(), 8);
    assert_eq!(stage.height(), 6);
    assert_eq!(stage.len(), 8 * 6);

    stage.clear([0, 0, 0, 255]);

    for i in 0..6 {
        stage.set_pixel(i, i, [255, 0, 0, 255]);
    }

    assert_eq!(
        stage.get_pixel(2, 2),
        Some([255, 0, 0, 255])
    );

    assert_eq!(stage.get_pixel(100, 100), None);

    let bytes = stage.as_bytes();
    assert_eq!(bytes.len(), stage.len() * 4);

    println!("smoke test passed");
}

