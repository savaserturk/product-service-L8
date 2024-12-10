use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Logitech Wireless Mouse M325".to_string(),
            price: 19.99,
            description: "Experience seamless wireless control with the Logitech M325, designed for smooth scrolling and long battery life.".to_string(),
            image: "/1.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Samsung Galaxy Buds 2 Pro".to_string(),
            price: 199.99,
            description: "Immerse yourself in high-quality sound with Samsung Galaxy Buds 2 Pro, offering noise cancellation and comfortable design.".to_string(),
            image: "/2.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Apple iPhone 15 Pro".to_string(),
            price: 1099.99,
            description: "Experience cutting-edge technology with the iPhone 15 Pro, featuring advanced performance, a stunning camera, and seamless iOS integration.".to_string(),
            image: "/3.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 1299.99,
            description: "The Dell XPS 13 combines sleek design, powerful performance, and a stunning display for all your productivity and entertainment needs.".to_string(),
            image: "/4.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Sony WH-1000XM5 Headphones".to_string(),
            price: 349.99,
            description: "Enjoy industry-leading noise cancellation and crystal-clear audio with Sony WH-1000XM5 headphones.".to_string(),
            image: "/5.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Samsung 55\" 4K UHD Smart TV".to_string(),
            price: 699.99,
            description: "Immerse yourself in stunning visuals and vibrant colors with the Samsung 55\" 4K UHD Smart TV.".to_string(),
            image: "/6.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Microsoft Surface Pro 9".to_string(),
            price: 999.99,
            description: "Work and play with versatility using the Microsoft Surface Pro 9, offering powerful performance in a portable design.".to_string(),
            image: "/7.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Google Nest Learning Thermostat".to_string(),
            price: 249.99,
            description: "Save energy and stay comfortable with the Google Nest Learning Thermostat, designed to adapt to your preferences.".to_string(),
            image: "/8.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Bose SoundLink Revolve+ Speaker".to_string(),
            price: 299.99,
            description: "Fill your space with immersive, 360-degree sound using the Bose SoundLink Revolve+ Bluetooth speaker.".to_string(),
            image: "/9.jpg".to_string()
        },
        Product {
            id: 10,
            name: "HP Envy All-in-One Printer".to_string(),
            price: 179.99,
            description: "Print, scan, and copy with ease using the HP Envy All-in-One Printer, designed for home and office use.".to_string(),
            image: "/10.jpg".to_string()
        }
    ]
}
