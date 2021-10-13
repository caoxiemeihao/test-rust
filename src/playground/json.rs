pub fn parse() {
    let parsed = json::parse(
        r#"
{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}
    "#,
    )
    .unwrap();

    println!("code: {}", parsed["code"]);
}
