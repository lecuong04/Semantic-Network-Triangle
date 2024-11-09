pub mod triangle {
    use std::{collections::HashSet, f64};

    pub const N_COLS: usize = 26;
    pub const N_ROWS: usize = 13;

    #[derive(Hash, Eq, PartialEq)]
    struct Pair {
        key: usize,
        value: usize,
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub struct Triangle {
        A: f64,
        B: f64,
        C: f64,
        a: f64,
        b: f64,
        c: f64,
        S: f64,
        hA: f64,
        hB: f64,
        hC: f64,
        P: f64,
        R: f64,
        r: f64,
        data: [[i8; N_COLS]; N_ROWS],
        root: Vec<usize>,
        isCalculated: bool,
        border: HashSet<Pair>,
        isError: bool,
        history: Vec<[usize; 2]>,
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    impl Triangle {
        pub fn New() -> Self {
            let mut t = Triangle {
                A: 0.0,
                B: 0.0,
                C: 0.0,
                a: 0.0,
                b: 0.0,
                c: 0.0,
                S: 0.0,
                hA: 0.0,
                hB: 0.0,
                hC: 0.0,
                P: 0.0,
                R: 0.0,
                r: 0.0,
                data: [[0; N_COLS]; N_ROWS],
                root: Vec::new(),
                isCalculated: false,
                isError: false,
                history: Vec::new(),
                border: HashSet::new(),
            };
            t.data[0][0] = -1;
            t.data[1][0] = -1;
            t.data[3][0] = -1;
            t.data[4][0] = -1;
            t.data[1][1] = -1;
            t.data[2][1] = -1;
            t.data[4][1] = -1;
            t.data[4][1] = -1;
            t.data[0][2] = -1;
            t.data[2][2] = -1;
            t.data[3][2] = -1;
            t.data[5][2] = -1;
            t.data[3][3] = -1;
            t.data[4][3] = -1;
            t.data[5][3] = -1;
            t.data[6][3] = -1;
            t.data[3][4] = -1;
            t.data[6][4] = -1;
            t.data[7][4] = -1;
            t.data[4][5] = -1;
            t.data[6][5] = -1;
            t.data[8][5] = -1;
            t.data[5][6] = -1;
            t.data[6][6] = -1;
            t.data[9][6] = -1;
            t.data[0][7] = -1;
            t.data[1][7] = -1;
            t.data[2][7] = -1;
            t.data[2][8] = -1;
            t.data[3][8] = -1;
            t.data[4][8] = -1;
            t.data[6][8] = -1;
            t.data[1][9] = -1;
            t.data[3][9] = -1;
            t.data[5][9] = -1;
            t.data[6][9] = -1;
            t.data[0][10] = -1;
            t.data[4][10] = -1;
            t.data[5][10] = -1;
            t.data[6][10] = -1;
            t.data[3][11] = -1;
            t.data[4][11] = -1;
            t.data[5][11] = -1;
            t.data[10][11] = -1;
            t.data[0][12] = -1;
            t.data[3][12] = -1;
            t.data[11][12] = -1;
            t.data[1][13] = -1;
            t.data[4][13] = -1;
            t.data[11][13] = -1;
            t.data[2][14] = -1;
            t.data[5][14] = -1;
            t.data[11][14] = -1;
            t.data[3][15] = -1;
            t.data[4][15] = -1;
            t.data[5][15] = -1;
            t.data[6][15] = -1;
            t.data[11][15] = -1;
            t.data[6][16] = -1;
            t.data[10][16] = -1;
            t.data[12][16] = -1;
            t.data[2][17] = -1;
            t.data[4][17] = -1;
            t.data[7][17] = -1;
            t.data[1][18] = -1;
            t.data[5][18] = -1;
            t.data[7][18] = -1;
            t.data[2][19] = -1;
            t.data[3][19] = -1;
            t.data[8][19] = -1;
            t.data[0][20] = -1;
            t.data[5][20] = -1;
            t.data[8][20] = -1;
            t.data[0][21] = -1;
            t.data[4][21] = -1;
            t.data[9][21] = -1;
            t.data[1][22] = -1;
            t.data[3][22] = -1;
            t.data[9][22] = -1;
            t.data[0][23] = -1;
            t.data[3][23] = -1;
            t.data[4][23] = -1;
            t.data[5][23] = -1;
            t.data[1][24] = -1;
            t.data[3][24] = -1;
            t.data[4][24] = -1;
            t.data[5][24] = -1;
            t.data[2][25] = -1;
            t.data[3][25] = -1;
            t.data[4][25] = -1;
            t.data[5][25] = -1;
            return t;
        }

        fn UpdateRow(&mut self, row: usize, value: f64) -> bool {
            if value <= 0.0 || value.is_infinite() || value.is_nan() {
                return false;
            }
            for i in 0..N_COLS {
                let curr: i8 = self.data[row][i];
                if curr == 1 {
                    break;
                } else if curr == -1 {
                    self.data[row][i] = 1;
                }
            }
            if !self.isCalculated {
                self.root.insert(self.root.len(), row);
            }
            return true;
        }

        fn TriggerCol(&self, col: usize) -> i32 {
            let mut result: i32 = 0;
            let mut row: i32 = -1;
            for i in 0..N_ROWS {
                if self.data[i][col] == -1 {
                    result += 1;
                    row = i as i32;
                }
            }
            if result == 1 {
                return row;
            } else {
                return -1;
            }
        }

        pub fn Set_A(&mut self, value: f64) {
            let value = value % 360.0;
            if value <= 0.0 || value == self.A {
                return;
            }
            if self.UpdateRow(0, value) {
                self.A = value;
            }
        }

        pub fn Get_A(&self) -> f64 {
            return self.A;
        }

        pub fn Set_B(&mut self, value: f64) {
            let value = value % 360.0;
            if value <= 0.0 || value == self.B {
                return;
            }
            if self.UpdateRow(1, value) {
                self.B = value;
            }
        }

        pub fn Get_B(&self) -> f64 {
            return self.B;
        }

        pub fn Set_C(&mut self, value: f64) {
            let value = value % 360.0;
            if value <= 0.0 || value == self.C {
                return;
            }
            if self.UpdateRow(2, value) {
                self.C = value;
            }
        }

        pub fn Get_C(&self) -> f64 {
            return self.C;
        }

        pub fn Set_a(&mut self, value: f64) {
            if value <= 0.0 || value == self.a {
                return;
            }
            if self.UpdateRow(3, value) {
                self.a = value;
            }
        }

        pub fn Get_a(&self) -> f64 {
            return self.a;
        }

        pub fn Set_b(&mut self, value: f64) {
            if value <= 0.0 || value == self.b {
                return;
            }
            if self.UpdateRow(4, value) {
                self.b = value;
            }
        }

        pub fn Get_b(&self) -> f64 {
            return self.b;
        }

        pub fn Set_c(&mut self, value: f64) {
            if value <= 0.0 || value == self.c {
                return;
            }
            if self.UpdateRow(5, value) {
                self.c = value;
            }
        }

        pub fn Get_c(&self) -> f64 {
            return self.c;
        }

        pub fn Set_S(&mut self, value: f64) {
            if value <= 0.0 || value == self.S {
                return;
            }
            if self.UpdateRow(6, value) {
                self.S = value;
            }
        }

        pub fn Get_S(&self) -> f64 {
            return self.S;
        }

        pub fn Set_hA(&mut self, value: f64) {
            if value <= 0.0 || value == self.hA {
                return;
            }
            if self.UpdateRow(7, value) {
                self.hA = value;
            }
        }

        pub fn Get_hA(&self) -> f64 {
            return self.hA;
        }

        pub fn Set_hB(&mut self, value: f64) {
            if value <= 0.0 || value == self.hB {
                return;
            }
            if self.UpdateRow(8, value) {
                self.hB = value;
            }
        }

        pub fn Get_hB(&self) -> f64 {
            return self.hB;
        }

        pub fn Set_hC(&mut self, value: f64) {
            if value <= 0.0 || value == self.hC {
                return;
            }
            if self.UpdateRow(9, value) {
                self.hC = value;
            }
        }

        pub fn Get_hC(&self) -> f64 {
            return self.hC;
        }

        pub fn Set_P(&mut self, value: f64) {
            if value <= 0.0 || value == self.P {
                return;
            }
            if self.UpdateRow(10, value) {
                self.P = value;
            }
        }

        pub fn Get_P(&self) -> f64 {
            return self.P;
        }

        pub fn Set_R(&mut self, value: f64) {
            if value <= 0.0 || value == self.R {
                return;
            }
            if self.UpdateRow(11, value) {
                self.R = value;
            }
        }

        pub fn Get_R(&self) -> f64 {
            return self.R;
        }

        pub fn Set_r(&mut self, value: f64) {
            if value <= 0.0 || value == self.r {
                return;
            }
            if self.UpdateRow(12, value) {
                self.r = value;
            }
        }

        pub fn Get_r(&self) -> f64 {
            return self.r;
        }

        fn GetHistoryKeys(&self) -> Vec<usize> {
            let mut result: Vec<usize> = Vec::new();
            for i in &self.history {
                result.push(i[0]);
            }
            return result;
        }

        pub fn TryGetHistoryValue(&self, key: usize, value: &mut usize, index: &mut usize) -> bool {
            *index = 0;
            for i in &self.history {
                *index += 1;
                if i[0] == key {
                    *value = i[1];
                    return true;
                }
            }
            return false;
        }

        pub fn Calculate(&mut self) {
            self.isError = false;
            self.isCalculated = true;
            let mut tryAgain: bool = false;
            let isInvalidValue = |value: f64| {
                if value.is_nan() || value.is_infinite() {
                    true
                } else {
                    false
                }
            };
            for col in 0..N_COLS {
                let mut err: bool = false;
                if self.GetHistoryKeys().contains(&col) {
                    continue;
                }
                let row = self.TriggerCol(col);
                if row == -1 {
                    continue;
                }
                if self.a != 0.0 && self.b != 0.0 && self.c != 0.0 {
                    if !(self.a < self.b + self.c && self.b < self.a + self.c && self.c < self.a + self.b) {
                        self.isError = true;
                        break;
                    }
                }
                tryAgain = true;
                match col {
                    0 => match row {
                        0 => {
                            let result = (self.B.to_radians().sin() * self.a / self.b).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_A(result);
                        }
                        1 => {
                            let result = (self.A.to_radians().sin() * self.b / self.a).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        3 => {
                            let result = self.b * self.A.to_radians().sin() / self.B.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        4 => {
                            let result = self.a * self.B.to_radians().sin() / self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    1 => match row {
                        1 => {
                            let result = (self.C.to_radians().sin() * self.b / self.c).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        2 => {
                            let result = (self.B.to_radians().sin() * self.c / self.b).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        4 => {
                            let result = self.c * self.B.to_radians().sin() / self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        5 => {
                            let result = self.b * self.C.to_radians().sin() / self.B.to_radians().sin().asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    2 => match row {
                        0 => {
                            let result = (self.C.to_radians().sin() * self.a / self.c).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_A(result);
                        }
                        2 => {
                            let result = (self.A.to_radians().sin() * self.c / self.a).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        3 => {
                            let result = self.c * self.A.to_radians().sin() / self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        5 => {
                            let result = self.a * self.C.to_radians().sin() / self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    3 => match row {
                        6 => {
                            let p: f64 = (self.a + self.b + self.c) / 2.0;
                            let result = (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    4 => match row {
                        3 => {
                            let result = 2.0 * self.S / self.hA;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        6 => {
                            let result = self.hA / 2.0 * self.a;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        7 => {
                            let result = 2.0 * self.S / self.a;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hA(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    5 => match row {
                        4 => {
                            let result = 2.0 * self.S / self.hB;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        6 => {
                            let result = self.hB / 2.0 * self.b;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        8 => {
                            let result = 2.0 * self.S / self.b;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hB(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    6 => match row {
                        5 => {
                            let result = 2.0 * self.S / self.hC;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        6 => {
                            let result = self.hC / 2.0 * self.c;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        9 => {
                            let result = 2.0 * self.S / self.c;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hC(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    7 => match row {
                        0 => {
                            let result = 180.0 - self.B - self.C;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_A(result);
                        }
                        1 => {
                            let result = 180.0 - self.A - self.C;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        2 => {
                            let result = 180.0 - self.B - self.A;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    8 => match row {
                        2 => {
                            let result = (2.0 * self.S / (self.a * self.b)).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        3 => {
                            let result = 2.0 * self.S / (self.b * self.C.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        4 => {
                            let result = 2.0 * self.S / (self.a * self.C.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        6 => {
                            let result = 0.5 * self.a * self.b * self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    9 => match row {
                        1 => {
                            let result = (2.0 * self.S / (self.c * self.a)).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        3 => {
                            let result = 2.0 * self.S / (self.c * self.B.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        5 => {
                            let result = 2.0 * self.S / (self.a * self.B.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        6 => {
                            let result = 0.5 * self.a * self.c * self.B.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    10 => match row {
                        0 => {
                            let result = ((2.0 * self.S / (self.c * self.b)) as f64).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_A(result);
                        }
                        4 => {
                            let result = 2.0 * self.S / (self.c * self.A.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        5 => {
                            let result = 2.0 * self.S / (self.b * self.A.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        6 => {
                            let result = 0.5 * self.b * self.c * self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    11 => match row {
                        3 => {
                            let result = self.P - self.b - self.c;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        4 => {
                            let result = self.P - self.a - self.c;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        5 => {
                            let result = self.P - self.a - self.b;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        10 => {
                            let result = self.a + self.b + self.c;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_P(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    12 => match row {
                        0 => {
                            let result = (self.a / (2.0 * self.R)).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_A(result);
                        }
                        3 => {
                            let result = 2.0 * self.R * self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        11 => {
                            let result = self.a / (2.0 * self.A.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_R(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    13 => match row {
                        1 => {
                            let result = (self.b / (2.0 * self.R)).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        4 => {
                            let result = 2.0 * self.R * self.B.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        11 => {
                            let result = self.b / (2.0 * self.B.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_R(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    14 => match row {
                        2 => {
                            let result = (self.c / (2.0 / self.R)).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        5 => {
                            let result = 2.0 * self.R * self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        11 => {
                            let result = self.c / (2.0 * self.C.to_radians().sin());
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_R(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    15 => match row {
                        3 => {
                            let result = 4.0 * self.S * self.R / (self.b * self.c);
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        4 => {
                            let result = 4.0 * self.S * self.R / (self.a * self.c);
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        5 => {
                            let result = 4.0 * self.S * self.R / (self.b * self.a);
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        6 => {
                            let result = self.a * self.b * self.c / (4.0 * self.R);
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        11 => {
                            let result = self.a * self.b * self.c / (4.0 * self.S);
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_R(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    16 => match row {
                        6 => {
                            let result = self.P * self.r / 2.0;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_S(result);
                        }
                        10 => {
                            let result = 2.0 * self.S / self.r;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_P(result);
                        }
                        12 => {
                            let result = 2.0 * self.S / self.P;
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_r(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    17 => match row {
                        2 => {
                            let result = (self.hA / self.b).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        4 => {
                            let result = self.hA / self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        7 => {
                            let result = self.b * self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hA(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    18 => match row {
                        1 => {
                            let result = (self.hC / self.c).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        5 => {
                            let result = self.hA / self.B.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        7 => {
                            let result = self.c * self.B.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hA(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    19 => match row {
                        2 => {
                            let result = (self.hB / self.a).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        3 => {
                            let result = self.hB / self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        8 => {
                            let result = self.a * self.C.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hB(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    20 => match row {
                        0 => {
                            let result = (self.hB / self.c).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_A(result);
                        }
                        5 => {
                            let result = self.hB / self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        8 => {
                            let result = self.c * self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hB(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    21 => match row {
                        0 => {
                            let result = (self.hC / self.b).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        4 => {
                            let result = self.hC / self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        9 => {
                            let result = self.b * self.A.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hC(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    22 => match row {
                        1 => {
                            let result = (self.hC / self.a).asin().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        3 => {
                            let result = self.hC / self.B.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        9 => {
                            let result = self.a * self.B.to_radians().sin();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_hC(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    23 => match row {
                        0 => {
                            let result = ((self.b.powi(2) + self.c.powi(2) - self.a.powi(2)) / (2.0 * self.b * self.c)).acos().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_A(result);
                        }
                        3 => {
                            let result = (self.b.powi(2) + self.c.powi(2) - (2.0 * self.b * self.c * self.A.to_radians().cos())).sqrt();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_a(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    24 => match row {
                        1 => {
                            let result = ((self.a.powi(2) + self.c.powi(2) - self.b.powi(2)) / (2.0 * self.a * self.c)).acos().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_B(result);
                        }
                        4 => {
                            let result = (self.a.powi(2) + self.c.powi(2) - (2.0 * self.a * self.c * self.B.to_radians().cos())).sqrt();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_b(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    25 => match row {
                        2 => {
                            let result = ((self.a.powi(2) + self.b.powi(2) - self.c.powi(2)) / (2.0 * self.a * self.b)).acos().to_degrees();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_C(result);
                        }
                        5 => {
                            let result = (self.a.powi(2) + self.b.powi(2) - (2.0 * self.a * self.b * self.C.to_radians().cos())).sqrt();
                            if isInvalidValue(result) {
                                err = true;
                            }
                            self.Set_c(result);
                        }
                        _ => {
                            if !self.border.insert(Pair { key: col, value: row as usize }) {
                                self.isError = true;
                            }
                            continue;
                        }
                    },
                    _ => {}
                }
                if !err {
                    self.history.push([col, row as usize]);
                } else {
                    if !self.border.insert(Pair { key: col, value: row as usize }) {
                        self.isError = true;
                    }
                }
            }
            if self.isError {
                return;
            }
            if tryAgain {
                self.Calculate();
            }
            if !(self.a < self.b + self.c && self.b < self.a + self.c && self.c < self.a + self.b) {
                self.isError = true;
            }
        }

        pub fn IsCalculated(&self) -> bool {
            return self.isCalculated;
        }

        pub fn IsError(&self) -> bool {
            return self.isError;
        }

        pub fn Root(&self) -> Vec<usize> {
            return self.root.clone();
        }

        pub fn History(&self) -> Vec<[usize; 2]> {
            return self.history.clone();
        }

        pub fn Data(&self) -> Vec<Vec<i8>> {
            let mut result: Vec<Vec<i8>> = Vec::with_capacity(N_ROWS);
            for row in 0..N_ROWS {
                let mut tmp: Vec<i8> = Vec::with_capacity(N_COLS);
                for col in 0..N_COLS {
                    tmp.insert(tmp.len(), self.data[row][col]);
                }
                result.insert(result.len(), tmp);
            }
            return result;
        }

        pub fn DebugMatrix(&self) {
            for row in 0..N_ROWS {
                for col in 0..N_COLS {
                    print!("{:>4}", self.data[row][col]);
                }
                println!()
            }
            println!()
        }

        pub fn DebugHistory(&self) {
            for i in &self.history {
                print!("[{}-{}] ", i[0], i[1]);
            }
        }

        pub fn DebugValues(&self) {
            println!("A = {}", self.A);
            println!("B = {}", self.B);
            println!("C = {}", self.C);
            println!("a = {}", self.a);
            println!("b = {}", self.b);
            println!("c = {}", self.c);
            println!("S = {}", self.S);
            println!("hA = {}", self.hA);
            println!("hB = {}", self.hB);
            println!("hC = {}", self.hC);
            println!("P = {}", self.P);
            println!("R = {}", self.R);
            println!("r = {}", self.r);
        }
    }
}

#[test]
fn test_triangle() {
    use crate::Triangle;

    let mut t = Triangle::New();
    t.Calculate();
    t.DebugValues();
    t.DebugHistory();
    println!("Error: {}", t.IsError());
}
