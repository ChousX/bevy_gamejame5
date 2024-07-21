use crate::prelude::*;

pub const FIRST_NAMES: [&str; 130] = [
        "Alice", "Bob", "Charlie", "David", "Emma", "Frank", "Grace", "Henry", "Ivy", "Jack",
        "Katherine", "Liam", "Mia", "Noah", "Olivia", "Peter", "Quinn", "Rachel", "Samuel",
        "Taylor", "Uma", "Victor", "Wendy", "Xander", "Yasmine", "Zachary", "Abigail", "Benjamin",
        "Charlotte", "Daniel", "Emily", "Finn", "Gabriella", "Hannah", "Isaac", "Julia", "Kevin",
        "Lily", "Michael", "Natalie", "Owen", "Penelope", "Quentin", "Rebecca", "Sophia", "Thomas",
        "Ursula", "Vincent", "Willow", "Xavier", "Yvonne", "Zoe", "Arthur", "Beatrice", "Caleb",
        "Diana", "Ethan", "Faith", "George", "Hazel", "Isaiah", "Jasmine", "Kenneth", "Lucy",
        "Marcus", "Nora", "Oscar", "Paige", "Quincy", "Rose", "Silas", "Tessa", "Uriel", "Violet",
        "Walter", "Xena", "Yara", "Zane", "Ava", "Bryan", "Chloe", "Dylan", "Ella", "Freya",
        "Gavin", "Harper", "Ian", "Jessica", "Kai", "Luna", "Mason", "Nora", "Oliver", "Piper",
        "Quinn", "Ryan", "Samantha", "Theodore", "Ursula", "Victor", "Willa", "Xander", "Yasmine",
        "Zara", "Andrew", "Bethany", "Christopher", "Delilah", "Elijah", "Fiona", "Gideon",
        "Hailey", "Isabella", "Jacob", "Kylie", "Landon", "Madison", "Nathan", "Olivia", "Peyton",
        "Quincy", "Rachel", "Sebastian", "Tiffany", "Ulysses", "Vanessa", "Wyatt", "Ximena",
        "Yvonne", "Zachariah"
];

pub const LAST_NAMES: [&str; 200] = [
        "Smith", "Johnson", "Williams", "Jones", "Brown", "Davis", "Miller", "Wilson", "Moore", 
        "Taylor", "Anderson", "Thomas", "Jackson", "White", "Harris", "Martin", "Thompson", 
        "Garcia", "Martinez", "Robinson", "Clark", "Rodriguez", "Lewis", "Lee", "Walker", 
        "Hall", "Allen", "Young", "Hernandez", "King", "Wright", "Lopez", "Hill", "Scott", 
        "Green", "Adams", "Baker", "Gonzalez", "Nelson", "Carter", "Mitchell", "Perez", 
        "Roberts", "Turner", "Phillips", "Campbell", "Parker", "Evans", "Edwards", "Collins", 
        "Stewart", "Sanchez", "Morris", "Rogers", "Reed", "Cook", "Morgan", "Bell", "Murphy", 
        "Bailey", "Rivera", "Cooper", "Richardson", "Cox", "Howard", "Ward", "Torres", "Peterson", 
        "Gray", "Ramirez", "James", "Watson", "Brooks", "Kelly", "Sanders", "Price", "Bennett", 
        "Wood", "Barnes", "Ross", "Henderson", "Coleman", "Jenkins", "Perry", "Powell", "Long", 
        "Patterson", "Hughes", "Flores", "Washington", "Butler", "Simmons", "Foster", "Gonzales", 
        "Bryant", "Alexander", "Russell", "Griffin", "Diaz", "Hayes", "Myers", "Ford", "Hamilton", 
        "Graham", "Sullivan", "Wallace", "Woods", "Cole", "West", "Jordan", "Owens", "Reynolds", 
        "Fisher", "Ellis", "Harrison", "Gibson", "Mcdonald", "Cruz", "Marshall", "Ortiz", "Gomez", 
        "Murray", "Freeman", "Wells", "Webb", "Simpson", "Stevens", "Tucker", "Porter", "Hunter", 
        "Hicks", "Crawford", "Henry", "Boyd", "Mason", "Morales", "Kennedy", "Warren", "Dixon", 
        "Ramos", "Reyes", "Burns", "Gordon", "Shaw", "Holmes", "Rice", "Robertson", "Hunt", "Black", 
        "Daniels", "Palmer", "Mills", "Nichols", "Grant", "Knight", "Ferguson", "Rose", "Stone", 
        "Hawkins", "Dunn", "Perkins", "Hudson", "Spencer", "Gardner", "Stephens", "Payne", "Pierce", 
        "Berry", "Matthews", "Arnold", "Wagner", "Willis", "Ray", "Watkins", "Olson", "Carroll", 
        "Duncan", "Snyder", "Hart", "Cunningham", "Bradley", "Lane", "Andrews", "Ruiz", "Harper", 
        "Fox", "Riley", "Armstrong", "Carpenter", "Weaver", "Greene", "Lawrence", "Elliott", 
        "Chavez", "Sims", "Austin", "Peters", "Kelley", "Franklin", "Lawson"
];

#[derive(Component)]
pub struct LastName(pub &'static str);

#[derive(Component)]
pub struct FirstName(pub &'static str);
