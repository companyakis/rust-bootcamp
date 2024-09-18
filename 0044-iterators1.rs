fn main() {

    let years: [u16; 6] = [2018, 2019, 2020, 2021, 2022, 2023];

    let iter_element = years.iter();

    for e in iter_element {

        print!("{e} "); // 2018 2019 2020 2021 2022 2023

    }

    // but we know "for loop" does same thing!
    // wait for the upcoming parts...

    for year in &years {
        print!("{year} "); // 2018 2019 2020 2021 2022 2023
    }

}


