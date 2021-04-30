use std::process::Command;

pub fn label_image(img_path: &str) -> Vec<String> {
    let output = Command::new("python3")
        .arg("src/google_api.py").arg(img_path).arg("clientsecret.json").output().expect("failed to execute process");

    if output.status.success() {
        return String::from_utf8_lossy(&output.stdout)
            .replace('\n',"").split('|')
            .into_iter().map(|x| String::from(x))
            .collect::<Vec<String>>();
    }
    return vec!();
}