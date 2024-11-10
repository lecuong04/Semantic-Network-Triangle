mod triangle;

extern crate tera;

use serde::Serialize;
use tera::{Context, Tera};
use triangle::triangle::*;

#[allow(non_snake_case)]
#[derive(Serialize)]
struct ViewModel {
    cA: f64,
    cB: f64,
    cC: f64,
    sA: f64,
    sB: f64,
    sC: f64,
    S: f64,
    hA: f64,
    hB: f64,
    hC: f64,
    P: f64,
    pR: f64,
    iR: f64,
    html: String,
}

#[allow(dead_code)]
fn html_builder_stable(triangle: &Triangle) -> String {
    let mut result = String::new();
    if triangle.IsError() {
        result.push_str("<div class=\"row\"><div class=\"col\"><h1 class=\"align-middle fst-italic mb-3 text-center text-danger\">Tam giác không hợp lệ!</h1></div></div>");
    } else {
        let mut properties: String = String::new();
        let mut formula: String = String::new();
        let mut table: String = String::new();

        let root = triangle.Root();
        properties.push_str("<div class=row><div class=col><nav aria-label=breadcrumb style=\"--bs-breadcrumb-divider:''\"><ol class=\"breadcrumb fs-5\"style=\"justify-content:center\"><li class=breadcrumb-item>Thuộc tính:</li>");
        if root.len() == 0 {
            properties.push_str("<li class=\"breadcrumb-item\">Rỗng</li>");
        } else {
            for p in root {
                properties.push_str("<li class=\"breadcrumb-item\">");
                match p {
                    0 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>A</mi></math>");
                    }
                    1 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>B</mi></math>");
                    }
                    2 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>C</mi></math>");
                    }
                    3 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>a</mi></math>");
                    }
                    4 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>b</mi></math>");
                    }
                    5 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>c</mi></math>");
                    }
                    6 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>S</mi></math>");
                    }
                    7 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>A</mi></msub></math>");
                    }
                    8 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>B</mi></msub></math>");
                    }
                    9 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>C</mi></msub></math>");
                    }
                    10 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>P</mi></math>");
                    }
                    11 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>R</mi></math>");
                    }
                    12 => {
                        properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>r</mi></math>");
                    }
                    _ => {}
                }
                properties.push_str("</li>");
            }
        }
        properties.push_str("</ol></nav></div></div>");
        result.push_str(&properties);

        let history = triangle.History();
        formula.push_str("<div class=\"row\"><div class=\"col\"><nav style=\"--bs-breadcrumb-divider:'>'\" aria-label=\"breadcrumb\"><ol class=\"breadcrumb fs-5\" style=\"justify-content:center\"><li class=\"breadcrumb-item\">Công thức</li>");
        if history.len() == 0 {
            formula.push_str("<li class=\"breadcrumb-item\">Rỗng</li>");
        } else {
            for h in &history {
                formula.push_str("<li class=\"breadcrumb-item\">");
                formula.push_str(format!("({})", h[0] + 1).as_str());
                match h[1] {
                    0 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>A</mi></math></sub>");
                    }
                    1 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>B</mi></math></sub>");
                    }
                    2 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>C</mi></math></sub>");
                    }
                    3 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>a</mi></math></sub>");
                    }
                    4 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>b</mi></math></sub>");
                    }
                    5 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>c</mi></math></sub>");
                    }
                    6 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>S</mi></math></sub>");
                    }
                    7 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>A</mi></msub></math></sub>");
                    }
                    8 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>B</mi></msub></math></sub>");
                    }
                    9 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>C</mi></msub></math></sub>");
                    }
                    10 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>P</mi></math></sub>");
                    }
                    11 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>R</mi></math></sub>");
                    }
                    12 => {
                        formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>r</mi></math></sub>");
                    }
                    _ => {}
                }
                formula.push_str("</li>");
            }
        }
        formula.push_str("</ol></nav></div></div>");
        result.push_str(&formula);

        table.push_str("<div class=\"row\"><div class=\"col\"><table><tr><th style=\"border-top:none;border-left:none\"></th>");
        for h in 0..N_COLS {
            table.push_str(format!("<th class=\"header\">({})</th>", h + 1).as_str());
        }
        table.push_str("</tr>");
        let data = triangle.Data();
        for row in 0..N_ROWS {
            table.push_str("<tr><td class=\"header\">");
            match row {
                0 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>A</mi></math>");
                }
                1 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>B</mi></math>");
                }
                2 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>C</mi></math>");
                }
                3 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>a</mi></math>");
                }
                4 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>b</mi></math>");
                }
                5 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>c</mi></math>");
                }
                6 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>S</mi></math>");
                }
                7 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>A</mi></msub></math>");
                }
                8 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>B</mi></msub></math>");
                }
                9 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>C</mi></msub></math>");
                }
                10 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>P</mi></math>");
                }
                11 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>R</mi></math>");
                }
                12 => {
                    table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>r</mi></math>");
                }
                _ => {}
            }
            table.push_str("</td>");
            for col in 0..N_COLS {
                let curr = data[row][col];
                if curr == 0 {
                    table.push_str(&format!("<td>{}</td>", curr).as_str());
                } else if curr == 1 {
                    let mut value: usize = 0;
                    let mut index: usize = 0;
                    if triangle.TryGetHistoryValue(col, &mut value, &mut index) && value == row {
                        table.push_str(&format!("<td class=\"fst-italic fw-bold text-success\">{}<sup class=\"text-success\">{}</sup></td>", curr, index).as_str());
                    } else {
                        table.push_str(&format!("<td class=\"fst-italic fw-bold text-primary\">{}</td>", curr).as_str());
                    }
                } else {
                    table.push_str(&format!("<td class=\"fst-italic fw-bold text-danger\">{}</td>", curr).as_str());
                }
            }
            table.push_str("</tr>");
        }
        table.push_str("</table></div></div>");
        result.push_str(&table);
    }

    return serde_json::to_string(&ViewModel {
        cA: triangle.Get_A(),
        cB: triangle.Get_B(),
        cC: triangle.Get_C(),
        sA: triangle.Get_a(),
        sB: triangle.Get_b(),
        sC: triangle.Get_c(),
        S: triangle.Get_S(),
        hA: triangle.Get_hA(),
        hB: triangle.Get_hB(),
        hC: triangle.Get_hC(),
        P: triangle.Get_P(),
        pR: triangle.Get_R(),
        iR: triangle.Get_r(),
        html: result,
    })
    .unwrap();
}

#[allow(dead_code)]
fn html_builder_dev(triangle: &Triangle) -> String {
    let mut result: String = String::new();
    if triangle.IsError() {
        result.push_str("<div class=\"row\"><div class=\"col\"><h1 class=\"align-middle fst-italic mb-3 text-center text-danger\">Tam giác không hợp lệ!</h1></div></div>");
    } else {
        #[derive(Serialize)]
        struct Template {
            properties: String,
            formula: String,
            table: String,
        }
        let mut template = Template {
            properties: String::new(),
            formula: String::new(),
            table: String::new(),
        };

        let root = triangle.Root();
        if root.len() == 0 {
            template.properties.push_str("<li class=\"breadcrumb-item\">Rỗng</li>");
        } else {
            for p in root {
                template.properties.push_str("<li class=\"breadcrumb-item\">");
                match p {
                    0 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>A</mi></math>");
                    }
                    1 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>B</mi></math>");
                    }
                    2 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>C</mi></math>");
                    }
                    3 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>a</mi></math>");
                    }
                    4 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>b</mi></math>");
                    }
                    5 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>c</mi></math>");
                    }
                    6 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>S</mi></math>");
                    }
                    7 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>A</mi></msub></math>");
                    }
                    8 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>B</mi></msub></math>");
                    }
                    9 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>C</mi></msub></math>");
                    }
                    10 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>P</mi></math>");
                    }
                    11 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>R</mi></math>");
                    }
                    12 => {
                        template.properties.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>r</mi></math>");
                    }
                    _ => {}
                }
                template.properties.push_str("</li>");
            }
        }

        let history = triangle.History();
        if history.len() == 0 {
            template.formula.push_str("<li class=\"breadcrumb-item\">Rỗng</li>");
        } else {
            for h in &history {
                template.formula.push_str("<li class=\"breadcrumb-item\">");
                template.formula.push_str(format!("({})", h[0] + 1).as_str());
                match h[1] {
                    0 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>A</mi></math></sub>");
                    }
                    1 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>B</mi></math></sub>");
                    }
                    2 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>C</mi></math></sub>");
                    }
                    3 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>a</mi></math></sub>");
                    }
                    4 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>b</mi></math></sub>");
                    }
                    5 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>c</mi></math></sub>");
                    }
                    6 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>S</mi></math></sub>");
                    }
                    7 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>A</mi></msub></math></sub>");
                    }
                    8 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>B</mi></msub></math></sub>");
                    }
                    9 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>C</mi></msub></math></sub>");
                    }
                    10 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>P</mi></math></sub>");
                    }
                    11 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>R</mi></math></sub>");
                    }
                    12 => {
                        template.formula.push_str("<sub><math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>r</mi></math></sub>");
                    }
                    _ => {}
                }
                template.formula.push_str("</li>");
            }
        }

        template.table.push_str("<tr><th style=\"border-top:none;border-left:none\"></th>");
        for h in 0..N_COLS {
            template.table.push_str(format!("<th class=\"header\">({})</th>", h + 1).as_str());
        }
        template.table.push_str("</tr>");
        let data = triangle.Data();
        for row in 0..N_ROWS {
            template.table.push_str("<tr><td class=\"header\">");
            match row {
                0 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>A</mi></math>");
                }
                1 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>B</mi></math>");
                }
                2 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>C</mi></math>");
                }
                3 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>a</mi></math>");
                }
                4 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>b</mi></math>");
                }
                5 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>c</mi></math>");
                }
                6 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>S</mi></math>");
                }
                7 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>A</mi></msub></math>");
                }
                8 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>B</mi></msub></math>");
                }
                9 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>h</mi><mi>C</mi></msub></math>");
                }
                10 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>P</mi></math>");
                }
                11 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>R</mi></math>");
                }
                12 => {
                    template.table.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>r</mi></math>");
                }
                _ => {}
            }
            template.table.push_str("</td>");
            for col in 0..N_COLS {
                let curr = data[row][col];
                if curr == 0 {
                    template.table.push_str(&format!("<td>{}</td>", curr).as_str());
                } else if curr == 1 {
                    let mut value: usize = 0;
                    let mut index: usize = 0;
                    if triangle.TryGetHistoryValue(col, &mut value, &mut index) && value == row {
                        template.table.push_str(&format!("<td class=\"fst-italic fw-bold text-success\">{}<sup class=\"text-success\">{}</sup></td>", curr, index).as_str());
                    } else {
                        template.table.push_str(&format!("<td class=\"fst-italic fw-bold text-primary\">{}</td>", curr).as_str());
                    }
                } else {
                    template.table.push_str(&format!("<td class=\"fst-italic fw-bold text-danger\">{}</td>", curr).as_str());
                }
            }
            template.table.push_str("</tr>");
        }

        let context = Context::from_serialize(template).unwrap();
        let render = Tera::default().render_str(include_str!(".\\template.html"), &context);
        result.push_str(&render.unwrap());
    }
    return serde_json::to_string(&ViewModel {
        cA: triangle.Get_A(),
        cB: triangle.Get_B(),
        cC: triangle.Get_C(),
        sA: triangle.Get_a(),
        sB: triangle.Get_b(),
        sC: triangle.Get_c(),
        S: triangle.Get_S(),
        hA: triangle.Get_hA(),
        hB: triangle.Get_hB(),
        hC: triangle.Get_hC(),
        P: triangle.Get_P(),
        pR: triangle.Get_R(),
        iR: triangle.Get_r(),
        html: result,
    })
    .unwrap();
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[allow(unused_variables)]
fn submit_triangle(ca: f64, cb: f64, cc: f64, sa: f64, sb: f64, sc: f64, s: f64, ha: f64, hb: f64, hc: f64, p: f64, pr: f64, ir: f64) -> String {
    let mut t = Triangle::New();
    t.Set_A(ca);
    t.Set_B(cb);
    t.Set_C(cc);
    t.Set_a(sa);
    t.Set_b(sb);
    t.Set_c(sc);
    t.Set_S(s);
    t.Set_hA(ha);
    t.Set_hB(hb);
    t.Set_hC(hc);
    t.Set_P(p);
    t.Set_R(pr);
    t.Set_r(ir);
    t.Calculate();
    return html_builder_dev(&t);
}

#[tauri::command]
fn new_triangle() -> String {
    let t = Triangle::New();
    return html_builder_dev(&t);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().plugin(tauri_plugin_shell::init()).invoke_handler(tauri::generate_handler![submit_triangle, new_triangle]).run(tauri::generate_context!()).expect("error while running tauri application");
}
