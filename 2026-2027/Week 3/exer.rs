#[derive(Debug, Clone, Copy)]
struct Point(f64, f64);

#[derive(Debug)]
enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, width: f64, height: f64 },
    Segment(Point, Point),
    PointObject(Point),
}

impl Shape {
    fn analyze_shape(&self) {
        match self {
            Shape::Circle { center, radius } => {
                let area = std::f64::consts::PI * radius * radius;
                println!(
                    "[CIRCLE] Center: ({}, {}), Radius: {}, Area: {:.2}",
                    center.0, center.1, radius, area
                );
            }
            Shape::Rectangle { top_left, width, height } => {
                let area = width * height;
                println!(
                    "[RECTANGLE] Top-left: ({}, {}), Width: {}, Height: {}, Area: {:.2}",
                    top_left.0, top_left.1, width, height, area
                );
            }
            Shape::Segment(p1, p2) => {
                println!(
                    "[SEGMENT] From point ({}, {}) to point ({}, {})",
                    p1.0, p1.1, p2.0, p2.1
                );
            }
            Shape::PointObject(p) => {
                println!("[POINT] Coordinates: ({}, {})", p.0, p.1);
            }
        }
    }
}

fn main() {
    // 1. Creating base points
    let origin = Point(0.0, 0.0);
    let p1 = Point(5.0, 10.0);
    let p2 = Point(2.5, 4.0);

    // 2. Initializing different shapes (Enum variants)
    let fig1 = Shape::PointObject(origin);
    let fig2 = Shape::Circle { center: p2, radius: 3.5 };
    let fig3 = Shape::Rectangle { top_left: origin, width: 10.0, height: 20.0 };
    let fig4 = Shape::Segment(origin, p1);

    // 3. Running the analysis
    println!("=== Geometric Shape Analyzer ===");
    fig1.analyze_shape();
    fig2.analyze_shape();
    fig3.analyze_shape(fig3);
    fig4.analyze_shape();
}