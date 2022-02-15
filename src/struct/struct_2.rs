#[derive(Debug)]
struct HinhChuNhat {
    dai: u32,
    rong: u32,
}

// constructor
impl HinhChuNhat {
    // method
    fn dien_tich_hcn(&self) -> u32 {
        return self.dai * self.rong;
    }

    fn chua(&self, hcn2: &HinhChuNhat) -> bool {
        return self.dai > hcn2.dai && self.rong > hcn2.rong;
    }
}

impl HinhChuNhat {
    fn hinh_vuong(kich_thuoc: u32) -> HinhChuNhat {
        return HinhChuNhat {
            dai: kich_thuoc,
            rong: kich_thuoc,
        };
    }
}

fn main() {
    let hcn: HinhChuNhat = HinhChuNhat { dai: 30, rong: 50 };
    let hcn2: HinhChuNhat = HinhChuNhat { dai: 20, rong: 40 };
    let hcn3: HinhChuNhat = HinhChuNhat { dai: 40, rong: 60 };
    let hcn4: HinhChuNhat = HinhChuNhat::hinh_vuong(30);

    println!("kich thuoc hcn: {:#?}", hcn);
    println!("dien tich hcn: {}", hcn.dien_tich_hcn());
    println!("hcn1 co chua hcn2: {}", hcn.chua(&hcn2));
    println!("hcn1 co chua hcn3: {}", hcn.chua(&hcn3));
    println!("kich thuoc hcn: {:#?}", hcn4);
}
