use image::Luma;
use qrcode::QrCode;
use sha256::digest;

#[cfg(not(tarpaulin_include))] // Exclude from coverage report.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    qr_generate(args);
}

fn qr_generate(args: Vec<String>) {
    // Get input from argument.

    if args.len() < 2 {
        panic!("Atleast one argument is required to generate QR code.")
    }
    let input = &args[1];

    // Generate QR code

    let code = QrCode::new(input.as_bytes()).unwrap();

    // If png argument given, save as png.

    if args.len() >= 3 && args[2] == "--png" {
        // Save as png with name of sha256 hash of input.

        let filename = create_filename(input);
        let image = code.render::<Luma<u8>>().build();

        image.save(&filename).unwrap();

        // Test that image has been created.

        println!("QR code saved as {}", filename);
    } else if args.len() >= 3 && args[2] != "--png" {
        // If unknown argument given, panic.

        panic!("Unknown argument {}.", args[2]);
    } else {
        // Print QR code to terminal.

        let qr_string = code
            .render()
            .dark_color('■')
            .light_color(' ')
            .quiet_zone(false)
            .build();

        println!("{}", qr_string);
    }

    return;
}

fn create_filename(input: &String) -> String {
    if input.is_empty() {
        panic!("Input is empty.")
    }
    let hash = digest(input.as_bytes());
    let filename = format!("{}.png", hash);
    return filename;
}

// Tests

mod tests {

    // Test that QR code is generated.
    #[test]
    fn test_qr_generate() {
        let args = vec![
            "qr_test".to_string(),
            "https://www.hofler.fi/fi/hofler-cowhide-primaloft.html".to_string(),
        ];
        super::qr_generate(args);
    }

    // Test that QR code is generated and saved as png.
    #[test]
    fn test_qr_generate_png() {
        let args = vec![
            "qr_test".to_string(),
            "https://www.hofler.fi/fi/hofler-cowhide-primaloft.html".to_string(),
            "--png".to_string(),
        ];
        super::qr_generate(args);
        assert!(std::path::Path::new(
            "1dfa29c04079750c4cf18257391164ae9626c5fc92f541e6ed6f4bc7e8355e9f.png"
        )
        .exists());
        // Remove testfile.
        let _remove: std::io::Result<()> = std::fs::remove_file(
            "1dfa29c04079750c4cf18257391164ae9626c5fc92f541e6ed6f4bc7e8355e9f.png",
        );
    }

    // Test that QR code is not generated with unknown argument.
    #[test]
    #[should_panic = "Unknown argument --test."]
    fn test_qr_generate_unknown_argument() {
        let args = vec![
            "qr_test".to_string(),
            "https://www.hofler.fi/fi/hofler-cowhide-primaloft.html".to_string(),
            "--test".to_string(),
        ];
        super::qr_generate(args);
    }

    // Test that QR code is not generated without input.
    #[test]
    #[should_panic = "Atleast one argument is required to generate QR code."]
    fn test_qr_generate_fail() {
        let args = vec!["qr_test_fail".to_string()];
        super::qr_generate(args);
    }

    // Test that filename is created.
    #[test]
    fn test_create_filename() {
        let input = "https://www.hofler.fi/fi/hofler-cowhide-primaloft.html".to_string();
        let filename = super::create_filename(&input);
        assert_eq!(
            filename,
            "1dfa29c04079750c4cf18257391164ae9626c5fc92f541e6ed6f4bc7e8355e9f.png"
        );
    }

    // Test that filename is not created without input.
    #[test]
    #[should_panic = "Input is empty."]
    fn test_create_filename_empty() {
        let input = "".to_string();
        let _filename = super::create_filename(&input);
    }
}
