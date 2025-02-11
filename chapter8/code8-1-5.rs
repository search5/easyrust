// 🚧
.for_each(|company| results_vec.push(company.get_ceo().ok_or("No CEO found")));