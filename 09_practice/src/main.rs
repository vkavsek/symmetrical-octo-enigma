use leptos::*;

const INITIAL_LIST_LENGTH: usize = 10;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

fn add_a_name(id: usize) -> String {
    #[rustfmt::skip]
    let names: Vec<&str> = vec!["Reyna Watts","Dakota Durham","Tiffany White","Aiden King","Victoria Conner","Phillip Horne","Marlowe Cortez","Zayn Walters","Samara Harrison","Gavin Figueroa","Lilith Payne","Edward Hensley","Malaya Hobbs","Brendan Bradshaw","Berkley Lamb","Kaysen McFarland","Annika Valdez","Kyler Wilkinson","Siena Santiago","Beckham Murphy","Bella Harvey","Cayden Yang","Angelina McKinney","Romeo Velazquez","Jaliyah Mendez","Arthur Nielsen","Vienna Webster","Shawn Short","Cheyenne Rodriguez","Henry Kirby","Skyla Christian","Ledger McDaniel","Dahlia Armstrong","Grant Hendricks","Dani Weeks","Anders Herrera","Ximena Middleton","Misael Levy","Flora Flowers","Saul Klein","Elianna Black","Matteo Olsen","Oaklyn Drake","Jalen Douglas","Aniyah Ryan","Timothy Reyes","Audrey Delarosa","Osiris Wyatt","Liberty Friedman","Darwin Owens","Amaya Fuller","Andre Glenn","Blaire Wolf","Jase Clements","Cara Gregory","Travis Sosa","Cassandra Franklin","Simon Diaz","Elena Casey","Armando Haynes","Lexi McKinney","Romeo Phillips","Naomi Sweeney","Nixon Wiggins","Capri Dennis","Emanuel Kane","Ellianna Ware","Tadeo Cordova","Florence Vo","Gordon Higgins","Leighton Bowman","Francisco David","Haylee Fitzgerald","Peyton Villalobos","Zoya Roy","Marcelo Tanner","Harmoni Glass","Allan O’Neal","Treasure McCann","Heath Yu","Navy Crosby","Tristen Stewart","Maya Serrano","Milan Campos","Sutton Nolan","Maximo Ayers","Simone Matthews","Preston Adkins","Emelia Shaffer","Dexter Butler","Athena Baker","Ezra Murphy","Bella Gonzalez","Ethan Finley","Jovie Blankenship","Ernesto Hull","Andi Knight","Beckett Anderson","Ella Tyler","Emmitt Thompson","Madison Moses","Niklaus Holmes","Bailey Wang","Cohen Rios","Brooke Velasquez","Sullivan Wong","Adelaide Palacios","Thaddeus Garcia","Amelia McCall","Kiaan Conway","Ryann Camacho","Tatum Pham","Raelyn Barber","Solomon Harrington","Legacy Foster","Kayden Robertson","Harmony Duncan","Avery Wagner","Maeve McBride","Denver McCullough","Hana Davila","Grey Fernandez","Amara Shepard","Damari Douglas","Aniyah Briggs","Case Hale","Brinley Pham","Russell Hodge","Coraline Portillo","Wallace Stephens","Millie Hickman","Jakobe Bradford","Rhea Fuentes","Bowen May","Adriana Summers","Darius Camacho","Armani Farrell","Ty Chang","Ophelia Calhoun","Gary Turner","Brooklyn Moore","Levi Brown","Charlotte Welch","Hendrix Frank","Dior Vazquez","Jesse Reeves","Lana Barrett","Angelo Galvan","Dallas Newman","Anderson Avalos","Paloma Watts","Dakota Wheeler","Sydney Salgado","Trace Kennedy","Brianna Medrano","Arian Rios","Brooke Jackson","Sebastian Wiley","Lauryn Church","Terrance Townsend","Azalea Gilmore","Vihaan Mora","Jemma Summers","Darius Bailey","Kennedy Delacruz","Memphis Aguirre","Ariah Velazquez","Drew Manning","Jennifer Guerrero","Bryce Blake","Amanda McCarthy","Devin Walters","Samara Zamora","Quentin Fernandez","Amara Vu","Kamdyn Pugh","Landry Espinosa","Khalid Pollard","Marisol Potter","Lucca Carr","Rowan Phan","Maison Phan","Elsa Pitts","Trey McCarthy","Kira Hawkins","Victor Bowers","Elisa Swanson","Hugo Houston","Lylah Tate","Dalton Lang","Amirah Hess","Lawrence Contreras","Daniela Person","Moses Clark","Chloe Wilcox","Jerry Park","Lia Atkins","Cason Roach","Lyanna Sanders","Jose Underwood","Ensley Lester","Lee Vu","Kimora Rios","Israel Booth","Zariyah Clements","Fisher McMahon","Belen Conrad","Dilan Cardenas","Raven McCarty","Blaise Khan","Mabel Cole","Nathaniel Lawrence","Lauren Kent","Mekhi Fuentes","Madeleine Osborne","Augustus Compton","Elina Henry","Carlos Adams","Stella Patterson","Amir Johnson","Emma Roth","Roy Bond","Alena Cuevas","Brecken Phelps","Laney Cross","Fabian Mullen","Shay Ramirez","David Santana","Myra Hartman","Baker Patel","Madeline Wolfe","Donovan Lindsey","Colette Kramer","Kylan Smith","Olivia Lane","Matias Phan","Elsa David","Alonso Stanton","Jaycee Spence","Cillian Ayers","Simone Ward","Jameson Holloway","Mae Burch","Gerald Lee","Scarlett Joseph","Kyle Chan","Hattie Rice","Graham Weaver","Teagan Fields","Clayton Reese","Rosemary Duarte","Abdullah Golden","Giuliana Gomez","Isaiah Smith","Olivia Byrd","Cristian McClain","Marleigh Schwartz","Edwin Glenn","Blaire Phillips","Andrew Xiong","Amayah Garza","Judah Rice","Ada Bernal","Eithan Joseph","Gracelynn Shaw","Elliot Proctor","Chandler Duncan","Avery Middleton","Madalyn Lambert","Mario Rose","Magnolia Kline","Ramon Beard","Ezra Moss","Porter Odom","Laylani Evans","Elias Flores","Emilia Harrison","Gavin Harper","Ana Collins","Miles Holmes","Bailey Moran","Tate Hansen","Hope Day","Kayson Ochoa","Luciana May","Finley Yu","Navy Cummings","Raiden Neal","Talia Solis","Ronin Love","Avianna Chapman","Knox Green","Zoe Walsh","Bodhi Gould","Violeta Howard","Jeremiah Arias","Aleah Skinner","Ridge Pitts","Nala Carter","Maverick Lin","Makenna Ballard","Kenzo McCarty","Halo Dougherty","Brett Johnson","Emma House","Yehuda Calderon","Serena Hubbard","Forrest Rodriguez","Evelyn Page","Pablo Walsh","Leia Mason","Brandon Travis","Mazikee Foster","Kayden Richard","Davina Archer","Ephraim Zuniga","Leslie Ingram","Tripp Richmond","Whitney Byrd","Cristian Thomas","Elizabeth Rollins","Wes Lee","Scarlett Calhoun","Gary Smith","Olivia Marin","Aldo McMahon","Belen Patton","Moises Armstrong","Presley Rivas","Damon Esparza","Ramona Thomas","Logan Hood","Briana Hines","Uriel Lopez","Gianna Boyle","Robin Guevara","Teresa Briggs","Case Estrada","Sawyer Diaz","Nathan Henderson","Maria Rich","Miller Hicks","Alina Everett","Camilo Lane","Amy Good","Davian Gardner","Jordyn Wong","Walter Sanford","Emerald Walls","Larry Hutchinson","Jamie Steele","Elian Conway","Ryann Collins","Miles Calhoun","Thalia Walls","Larry Gray","Sarah Mann","Nehemiah Shaffer","Alanna Bush","Tyson Gilmore","Chanel Bean","Mccoy Simon","Kalani Beard","Nathanael Munoz","Kehlani Carr","Kash Coffey","Paola Wood","Carson Morse","Kairi Lu","Duncan Jimenez","Adeline Patton","Moises Hanson","Mariana Welch","Hendrix Le","Myla Hunter","Archer Romero","Eliza Hurley","Van Bernal","Emmeline Montes","Darren Adams","Stella Lester","Lee Franklin","Angela Reed","Easton Hill","Hannah Franklin","Simon Maddox","Zainab Clayton","Boston Gillespie","Alianna Carey","Watson Cross","Nayeli Bradshaw","Emory Dunlap","Iliana Roth","Roy Fox","Juliette Calhoun","Gary Orr","Alaiya Snyder","Thiago Jarvis","Elisabeth Mitchell","Jaxon McCann","Joyce Proctor","Vance Dorsey","Addyson Miller","Benjamin Harrell","Kara Gutierrez","Luca Bowman","Fiona Galindo","Salvatore Bass","Zahra Guevara","Tommy Richard","Davina Valentine","Demetrius Cantrell","Yamileth Mendez","Arthur Jacobs","Camilla Love","Jeffrey Parrish","Tiana Shepard","Damari Lam","Karina Yates","Braylon McCann","Joyce Vaughn","Remy Drake","Jayleen Mayo","Jericho Fry","Clarissa Booker","Dominik Shannon","Harlee Weaver","Tucker Skinner","Mara Franco","Gage Villalobos","Zoya Huffman","Chris Thornton","Haisley Kline","Ramon Davis","Mia Villegas","Clyde Yoder","Emerie O’brien","Riley Horn","Avah Williamson","Emerson Guerra","Edith Bond","Roger Case","Cleo Medina","George Booker","Nataly Crosby","Tristen Simpson","Anastasia Mays","Jadiel Newton","Braelynn Humphrey","Krew Quinn","Heaven Keller","Nico Wheeler","Sydney McCormick","Jasiah Webb","Ariella Hurst","Neil Bradshaw","Berkley Martin","Mateo Conrad","Bexley Cortes","Banks Bauer","Haley Felix","Rodney Le","Myla Tyler","Emmitt Preston","Indie Burch","Gerald Larson","Alayna Randolph","Eugene Sanford","Emerald Odom","Kylian Armstrong","Presley McCormick","Jasiah Galvan","Dallas King","Julian Hendricks","Dani Odom","Kylian Rodgers","Selah Knox","Valentin Gilmore","Chanel Cain","Benson Zamora","Sierra Church","Terrance Rogers","Madelyn McCullough","Briar Holmes","Bailey Ochoa","Winston Swanson","Helen Robbins","Finnegan Camacho","Armani Hoffman","Steven Bullock","Winnie McPherson","Foster Lynch","Malia Rivera","Charles Hall","Leah Tran","Braxton Richard","Davina Hughes","Everett Figueroa","Lilith Bullock","Ben Cruz","Claire Perez","Owen Sullivan","Melanie Cherry","Rome Holmes","Bailey Walker","Luke Garrett","Tessa Cox","Connor Burton","Miriam Robinson","Matthew Arellano","Faye Watkins","Nash Stevens","Katherine Rodriguez","Henry McDonald","Daisy Ashley","Kylen Case","Cleo Simon","Zayne Pratt","Ailani Hodge","Reign Sloan","Selene Hernandez","Mason Bishop","Brooklynn Henson","Bellamy Ball","Abby Garza","Judah Branch","Luisa Pruitt","Gatlin Byrd","Giselle Doyle","Kashton Huff","Karsyn Espinoza","Dallas Brady","Ryan Crosby","Tristen Strickland","Nia Grant","Leon Beard","Ezra Cisneros","Alden Johns","Giovanna Jensen","Cash Golden","Giuliana Frank","Braylen Finley","Jovie Santiago","Beckham Gibbs","Carter McCoy","Jett Khan","Mabel Blackwell","Marcellus Burton","Miriam Newman","Anderson Mata","Lillie Marquez","Malakai Manning","Jennifer Jaramillo","Riggs Carey","Alora McCormick","Jasiah Woods","Reese Parrish","Karsyn Craig","Brynn Mills","Alex Fischer","Maci Wagner","Enzo Rice","Ada Wall","Issac Howell","Mckenna Dunlap","Aries Andersen","Zoie Murphy","Cameron Rocha","Emmie Chandler","Royal Gilmore","Chanel Nguyen","Gabriel Huerta","Dulce Craig","Odin Melton","Kamiyah Sanchez","Joseph Andrade","Emmy Gilbert","Tobias Serrano","Allie Hardy","Jayceon Roman","Astrid Santana","Mohamed Byrd","Giselle Gilbert","Tobias Strickland","Nia Hoffman","Steven Blackburn","Frida Chambers","Orion Shah","Angelica Grant","Leon Robbins","Stevie Rhodes","Titus Hodge","Coraline Ray","Arlo Zhang","Sarai Kemp","Melvin Munoz","Kehlani Cano","Terry Carlson","Kali Bullock","Ben Hensley","Malaya Murphy","Cameron Phan","Elsa Reilly","Alvaro Kirby","Skyla Baxter","Tomas Valenzuela","Henley Bartlett","Kace Andrews","Payton Soto","Barrett Carpenter","Lilly Castaneda","Collin Chavez","Nevaeh Arias","Alec Schmidt","Kimberly Long","Jace McLaughlin","Stephanie Huff","Finnley Alvarez","Leilani Valentine","Demetrius Le","Myla Reid","Josue Walters","Samara Simpson","Elliott Sanders","Everleigh Tapia","Samir Macdonald","Rosalia Cain","Benson Blankenship","Rosalee May","Finley Walter","Penny Small","Rudy Person","Dylan Rich","Miller Fuentes","Madeleine Young","Asher Villa","Johanna Hancock","Rex Krueger","Kamari Quinn","Rhys Portillo","Nathalie York","Leandro Holloway","Mae Miles","Jared Cruz","Claire Wolfe","Donovan Riley","Kayla Esparza","Carl Escobar","Erin Rivera","Charles Roman","Astrid Trejo","Wesson Robbins","Stevie Brooks","Jordan Krueger","Kamari Bell","Emmett Graham","Alaia Saunders","Kasen Bennett","Josephine Black","Matteo Huff","Karsyn Friedman","Darwin Lowery","Estelle Richmond","Mordechai Olson","Isabel Coleman","Micah Santos","Alana Le","Damien Whitaker","Ivanna Conner","Phillip Dickerson","Opal Aguirre","Andy Chan","Hattie Singleton","Landyn Gould","Violeta Kline","Ramon Kerr","Baylee Lowery","Jaxxon Greer","Reina Wu","Kyson Cohen","Destiny Moyer","Ahmir Peterson","Caroline Malone","Ruben Erickson","Sabrina Flynn","Kannon Bean","Jenesis Wilkerson","Carmelo Peralta","Malayah Barron","Dustin Owens","Amaya Lynch","Zane Higgins","Leighton Schroeder","Izaiah Garza","River Tapia","Samir Norris","Arielle Noble","Idris Barnes","Liliana Taylor","Jackson Bean","Jenesis Bryan","Jaxtyn Galvan","Dallas Ward","Jameson Potter","Rory Roman","Kian Richard","Davina Esquivel","Bridger Walsh","Leia Hutchinson","Korbin O’Donnell","Bellamy Collier","Edison Matthews","Lila Avalos","Coen Gilmore","Chanel Rivers","Bear Cook","Aaliyah Oliver","Karson Acosta","Kaia Barber","Solomon Donovan","Azariah Kline","Ramon Patel","Madeline Trevino","Jaime Esparza","Ramona Tanner","Bruno Schaefer","Mavis Chavez","Ian Escobar","Erin Cantrell","Harris Salas","Amber Walker","Luke Parks","Ainsley Sandoval","Brantley Melendez","Bethany Valencia","Dax Wells","Cecilia Pitts","Trey Mullins","Maliyah Blanchard","Adler Eaton","Miley Yoder","Johan Doyle","Annalise Gilmore","Vihaan Ruiz","Emery Alfaro","Xzavier Lozano","Cecelia Thornton","Malik Hahn","Fallon Mata","Ray Buchanan","Maryam Parker","Caleb Miles","Alessandra Barajas","Brennan Lowe","Amari Gallagher","Marcos Whitaker","Ivanna Kerr","Louie Best","Lexie Reyes","Eli Pugh","Landry Estes","Hakeem Cortes","Lea Savage","Keaton Hickman","Scarlette Jarvis","Marlon Armstrong","Presley Hebert","Guillermo Cantrell","Yamileth Dalton","Fletcher Lester","Averi Brock","Julio Black","Molly Pineda","Gerardo Marsh","Adelina Hutchinson","Korbin Pratt","Ailani Larson","Rafael Brewer","Thea Thomas","Logan Prince","Greta Stevenson","Callan Rocha","Emmie Miles","Jared Fletcher","Anaya Richmond","Mordechai Hickman","Scarlette Santana","Mohamed Patton","Lorelei Spence","Cillian Baldwin","Esmeralda Hensley","Layne Baldwin","Esmeralda Ferguson","Miguel Parks","Ainsley Walsh","Bodhi Benitez","Aliza Maldonado","Javier Novak","Kaiya Flowers","Saul Clay","Aliana Powell","Bennett Page","Cataleya Norton","Callen Cruz","Claire Cardenas","Johnathan Romero","Eliza Watson","Greyson Maynard","Carolyn Lim","Cal Vo","Artemis Roy","Marcelo Barton","Danna Cobb","Raphael Walton","Scarlet Ortega","Kobe Marquez","Milani Buckley","Aryan Garrison","Cadence Bernal","Eithan Elliott","Noelle Leblanc","Braden Velasquez","Esme Travis","Willie Fuller","Oakley Harrell","Nelson Murray","Faith Webster","Shawn Castaneda","Keira Doyle","Kashton Gordon","Taylor Aguirre","Andy Marshall","Adalyn Chambers","Orion Mendoza","Cora Blackwell","Marcellus Wells","Cecilia Vang","Jimmy Brooks","Autumn Moyer","Ahmir McIntosh","Gwen Morrow","Kyree Coffey","Paola Christian","Ledger Stanton","Jaycee Larson","Rafael Hancock","Katelyn Walker","Luke Wood","Natalia Perkins","Kyrie Matthews","Lila Hogan","Sonny Gillespie","Alianna Franklin","Simon Houston","Lylah Dalton","Fletcher Lopez","Gianna Miles","Jared Mayer","Ainhoa Morales","Aaron Hogan","Kathryn Thomas","Logan Wood","Natalia McKee","Bjorn Phan","Elsa Townsend","Alexis Roberts","Paisley Ballard","Kenzo Anderson","Ella Booker","Dominik Ponce","Aileen Zhang","Isaias Jackson","Avery Person","Moses Johnson","Emma Sosa","Emir Hardy","Jessica Hahn","Kabir Boyer","Chaya Bravo","Genesis Vaughn","Reign Hampton","Hank Medrano","Halle Weiss","Koa Barrera","Beatrice Holmes","King Boone","Mariam Quintero","Thatcher Aguilar","Josie Glass","Allan Cooper","Serenity Matthews","Preston Randall","Christina Rojas","Colin Montes","Roselyn Quintana","Kelvin Griffin","Charlie Higgins","Sterling Friedman","Aspyn Hughes","Everett Schroeder","Cameron Durham","Kellen Vance","Maxine Dean","Ronan Macdonald","Rosalia Gentry","Magnus Rios","Brooke Zhang","Isaias Schultz","Briella Norman","Aziel Yu","Navy Ellison","Kye Young","Zoey Bridges","Mohammed Ashley","Khalani Fleming","Fernando Chan","Hattie Vu","Kamdyn Rodriguez","Evelyn Robinson","Matthew Bartlett","Aubrielle Galvan","Kingsley Reilly","Tori Rush","Kaiser Sheppard","Veda Vincent","Aarav Walton","Scarlet McCall","Kiaan Serrano","Allie Garrett","Kairo Gallagher","Elliott Curtis","Muhammad Page","Cataleya McGee","Conner Hutchinson","Jamie Salas","Zaiden Moyer","Zola Prince","Aron Marks","Monica Mays","Jadiel Ahmed","Jolie Hampton","Hank Davidson","Jayla Pacheco","Erik Stephenson","Khaleesi Travis","Willie Duarte","Kynlee Hendrix","Korbyn Friedman","Aspyn Moody","Ryland Graham","Alaia Perkins","Kyrie Nielsen","Vienna Hernandez","Mason Winters","Kataleya Wang","Cohen McBride","Kelsey Pena","Marcus Chavez","Nevaeh Duran","Ismael Lyons","Kenzie Richard","Ahmed Bravo","Amoura Webster","Shawn Noble","Hunter Potts","Alfred Woods","Reese Pena","Marcus Zamora","Sierra Fitzgerald","Peyton Dodson","Etta Dennis","Emanuel Fitzpatrick","Annabella McLaughlin","Ibrahim Good","Nathalia Suarez","Soren Fuller","Oakley Gilmore","Vihaan Brewer","Thea Beck","Eduardo Barker","Remington McConnell","London Kennedy","Brianna Ibarra","Asa Pacheco","Paris Charles","Conrad Bruce","Marilyn Gallegos","Jonas Hendricks","Dani Barr","Harley McGee","Kayleigh Hoffman","Steven Pennington","Yareli Villegas","Clyde Hensley","Malaya Clarke","Stetson Owens","Amaya Patton","Moises Guevara","Teresa Browning","Rohan Gilbert",];

    names[id].to_owned()
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Demo"</h1>
        <MakeList/>
    }
}

#[component]
fn MakeList(cx: Scope) -> impl IntoView {
    let mut next_count_id = INITIAL_LIST_LENGTH;
    let list_elements: Vec<_> = (0..next_count_id)
        .map(|id| {
            (
                id,
                (create_signal(cx, id + 1), create_signal(cx, add_a_name(id))),
            )
        })
        .collect();

    let (wrap_list_count, set_wrap_list_count) = create_signal(cx, list_elements);

    let push_names = move |_| {
        (0..20).for_each(|_| {
            let sig = (
                create_signal(cx, next_count_id + 1),
                create_signal(cx, add_a_name(next_count_id)),
            );

            set_wrap_list_count.update(move |list_count| list_count.push((next_count_id, sig)));
            next_count_id += 1;
        });
    };

    view! { cx,
        <div>
            <button on:click=push_names>"Add 20 names"</button>
            <ul>
                <For
                    each=wrap_list_count
                    key=|counter| counter.0
                    view=move |cx, (id, ((n, set_n), (name, set_name)))| {
                        view! { cx,
                            <li>
                                <button on:click=move |_| {
                                    set_n
                                        .update(|num| {
                                            set_name(add_a_name(*num));
                                            *num += 1;
                                        });
                                }>"+1"</button>
                                <button on:click=move |_| set_wrap_list_count.update(|elem| elem.retain(|(count_id, _)| count_id != &id))>
                                    "Remove"
                                </button>
                                {n}
                                "-"
                                {name}
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
