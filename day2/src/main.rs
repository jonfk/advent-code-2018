
use std::collections::HashMap;

fn main() {
    println!("Hello, day 2!");

    let boxes = parse_input(INPUT);

    let counts: HashMap<u64, u64> = boxes.clone().into_iter().fold(HashMap::new(), |mut state, str_box| {
        let counts = check_box(&str_box);
        counts.into_iter().for_each(|count| {
            let total_count = state.entry(count).or_insert(0);
            *total_count += 1;
        });
        state
    });
    let checksum = counts.into_iter()
        .fold(1, |acc, (_k, v)| v * acc);

    println!("checksum: {}", checksum);

    for str_box in boxes.clone().into_iter() {
        if let Some(common_letters) = find_almost_same_box(&str_box, boxes.clone()) {
            println!("common letters: {}", common_letters);
            break;
        }
    }
}

pub fn find_almost_same_box(str_box: &str, boxes: Vec<String>) -> Option<String> {
    boxes.into_iter()
        .filter(|x| x != str_box)
        .find_map(|x| is_almost_same_box(str_box, &x))
}

pub fn is_almost_same_box(str_box: &str, other_box: &str) -> Option<String> {
    let mut str_box_iter = str_box.chars();
    let mut other_box_iter = other_box.chars();
    let mut common_letters = String::new();
    let mut differences = 0;

    loop {
        if let Some(char1) =  str_box_iter.next() {
            if let Some(char2) = other_box_iter.next() {
                if char1 != char2 {
                    differences += 1;
                } else {
                    common_letters.push(char1);
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if differences == 1 {
        println!("str1: {}, str2: {}", str_box, other_box);
        Some(common_letters)
    } else {
        None
    }
}

pub fn check_box(str_box: &str) -> Vec<u64> {
    let mut char_counts: HashMap<char, u64> = str_box.chars()
        .fold(HashMap::new(), |mut state, c| {
            {
            let count = state.entry(c).or_insert(0);
            *count += 1;
            }
            state
        });
    let mut counts:Vec<u64> = char_counts.drain().filter(|(_k,v)| *v > 1).map(|(_k,v)| {v}).collect();
    counts.sort();
    counts.dedup();
    counts
}

pub fn parse_input(input: &str) -> Vec<String> {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

static INPUT: &str = "prtkqyluibmtcwqaezjmhgfndx
prtkqylusbsmcwvaezjmhgfndt
prgkqyluibsocwvamzjmhgkndx
prjkqyluibsocwvahzjmhgfnsx
prtkqylcibsocwvzezjohgfndx
prtkqyluiksocwziezjmhgfndx
prikqyluiksocwvaezjmkgfndx
prtkgyluibsocwvwezjehgfndx
prtkqyluiysocwvaezjghxfndx
prtkqwluibsoxwvaezjmhgfhdx
prtkqylgibsocwvabzjmhzfndx
prtknyltibnocwvaezjmhgfndx
prdkqyluibrocwvaezjmhgnndx
prtwqyluibsoctvcezjmhgfndx
mrtkqyluibgocwvakzjmhgfndx
prtkqaouibsocwvaezjmhwfndx
prtkqyluihjocwvaezjmhgfpdx
prtkqyluikfxcwvaezjmhgfndx
prtkqybuixsocwvaczjmhgfndx
pvtkayluibsocwxaezjmhgfndx
grtkqgluibsocdvaezjmhgfndx
prlkqyluibsochvaezjmhgzndx
prtkqylxibsocmvaezjmhgfkdx
prtkqyluibsqctvaezjmpgfndx
putkqyluibsocqvaezjmhgfndw
prtjqyluibsiclvaezjmhgfndx
prtkqylvpvsocwvaezjmhgfndx
prnkqyluibsocwvaezjmhefsdx
prtktyluibsocwvaezjkhgrndx
prtkqyluibcovwvaezjthgfndx
prtkqcluibiocwvaezjmhggndx
prtkqyluihsocwveezjmhgfydx
prtklyluibsocwqaszjmhgfndx
prtkqyluibsocwvaezjmfznndx
prtkjyluijsocwvaeejmhgfndx
prtkqtluibsonwvaexjmhgfndx
prtkqyluinsocwbaezjmjgfndx
prtkqyluibslckvaezjmhgyndx
prtkqyluibsodwlpezjmhgfndx
prtkquluibsfcwvaezjhhgfndx
prtkqyluhbsocweaezsmhgfndx
prrkqyluinsocxvaezjmhgfndx
prtkqyluibsoswvaezjmhgyqdx
prtkqbluibdocwvlezjmhgfndx
prtkqyfuibsocpvaezjmhgfnwx
prtkqlluibsqjwvaezjmhgfndx
prtkqyluibrocwvaehjmjgfndx
prtkqyluibsoowvaezgmhgendx
wrtjqyluibsocwvaezfmhgfndx
prtvqyluhbsocwvaezjmtgfndx
prtkqyllibspcwvaezjmkgfndx
pqtzqyeuibsocwvaezjmhgfndx
prtkqyluibsolpvaezjmegfndx
przkayguibsocwvaezjmhgfndx
prtkqyluidsocwvaezjmyufndx
prtuqyluibsocwvaezjmfgfnkx
prtkqwluibsrcwvaezjchgfndx
prtkqyluibsotwhaozjmhgfndx
erwkqylhibsocwvaezjmhgfndx
prtkqyluibsocwvgezjmkgfedx
prskqyluiesocwvaezjmggfndx
prtkqylmitsocwvaezjmhgfnox
prtkqyluinnocwvaezjmhgfkdx
prtktyluibsokwvaezjmhgfcdx
prtkqyluibsomwvakvjmhgfndx
prtkqyltibloawvaezjmhgfndx
prtkqyluibxocwvaezgmhgqndx
prtkqyluibskcmvaezjmhgfngx
artkqylubbsotwvaezjmhgfndx
prtkqyluibzocwvhezjmhgfnbx
prskqkluibsocwvaezjmhgfjdx
prtkqyluibwocwvaezjkhglndx
prukqyluissocwvzezjmhgfndx
puhkqyluibsocwvaezjmhgfsdx
qrtkqyluibsocwvaeujmhgfndd
prtkqyluibsoctvaezjmagfnda
prtkquluibsocwkaezjmhgfqdx
prtkqyluubswcwvaezjmhvfndx
prfkqyluibsocwvaemrmhgfndx
pmtkqyluibpocwvaezjmhggndx
prtkqvluibiocwvaezjqhgfndx
prtkgypuibsocwvaezcmhgfndx
prtpqyquibsovwvaezjmhgfndx
prtwqyluiasocwvaexjmhgfndx
mrtzqyluibbocwvaezjmhgfndx
prtkqyluibsocwmaegwmhgfndx
prtkqyluibvncwvaqzjmhgfndx
prtkqyluiusocwvaezjmhmfbgx
prtkqyljibvocwvaezjehgfndx
prtkqyloibsopavaezjmhgfndx
prckqyakibsocwvaezjmhgfndx
prtkqyluibsdcwvaezjmngfddx
prekqylupbsocwvaezemhgfndx
hrtkqyluibhocwvaezjmhgfnde
prmkqyluibsocwvaezzfhgfndx
prtkqyluiccfcwvaezjmhgfndx
pdtkqyluxbsocwvaezjmhgendx
prokqyluibsocwvuezjmsgfndx
prtkqyluibsacwvaezjyhgfndv
prtkqmluibsocavaezjmhgfndc
prtkqyluibsocwvmezjmhgtnqx
prtkqytuibiocyvaezjmhgfndx
pktkqyiuibsocwvwezjmhgfndx
grtrqyluibsocwvaezjmhgfbdx
prtkqylsibjocwvaezjmhgfnyx
prtkqyhutbsocwvaexjmhgfndx
prtknyluibsocmvaezumhgfndx
prtkwyluibsocwvahzjmhgpndx
prtkqywuibsolhvaezjmhgfndx
prtkcyluibsoccvaezjthgfndx
prtkqyrdibsocwvaezjbhgfndx
prtkqyhuqbsocwvaezjmhgfxdx
pytkqyluibsocwvagzjmhgfndv
prtkqyliibsocwvaexwmhgfndx
prtkqyluibshcwvaeljphgfndx
prtkqyluibsocwvaerjzhbfndx
prtkqyduibsocwvaezvmhgfnzx
drtkqylhibsocwvaezjmhmfndx
prtkqyluibsocwvaezamfvfndx
brtkqyluqbsocwvaezjmhgpndx
prtkqyiuibsocwvuezjmhgfngx
urtkqyluibsocqvaeljmhgfndx
prtkqyluikaocwvaezjmhgfjdx
prqkqzouibsocwvaezjmhgfndx
prtkqyluibsocxvaezjmhgfnxv
prlkqyluibsoxwvaeijmhgfndx
prthuyluibsocwvaezjmhgfnhx
potkqyluizsocwvaezjmhifndx
fstkqyduibsocwvaezjmhgfndx
prtkqxluibsocwvaezjmhgffdm
prtkqylpibsozwvaezmmhgfndx
prxkqylbibsocwvaezjphgfndx
srtkqyluibsicnvaezjmhgfndx
prtktyluibsocwvaezjvhgfnax
pctkqyluxbsocwvaezwmhgfndx
prtkqylusbsoclvaezsmhgfndx
pwtkqyluibsocrvaezjmggfndx
prtkqyluibswcwraezjmhgfndd
prtkqyluibtocwiaezjmhgfnax
prtuqyluibsocwvajzjmngfndx
pwtkqyluibsocwvaerjmogfndx
petkqexuibsocwvaezjmhgfndx
pztkqyluibsocwvaerqmhgfndx
prtkqyluobsocwvaezjmapfndx
prtkqyluiinocwvaeljmhgfndx
prtkqyluibsoowvxezjmhgfnnx
lrtkqyluibsocwvfezjmhgfndc
prtkqyluibokcwvahzjmhgfndx
prtkqmlufbsocwvaegjmhgfndx
prtkqylribsocwvanzjmhgfnda
prtkqyluibspxwvaezkmhgfndx
prtiqyluibsbcwvaezjmhgfntx
prikqzluinsocwvaezjmhgfndx
prtkqnldibsocwvaezjmhxfndx
prtkqyluixsocsvaezjmhwfndx
hrtkqyluibsocwvaezjhhgfodx
prtkqyluibsrcwvaezjmhpfwdx
prtkqyluibsocwyaezjmhgffdk
prtkqyluidsocwvalmjmhgfndx
prukquluabsocwvaezjmhgfndx
prckqyluinsmcwvaezjmhgfndx
prbkqymuibsocwvaezjmhgfndc
prtkfylaibsocwvaezjmkgfndx
zrtkqyluibsocwvrbzjmhgfndx
crtkqyluibsocwvaejjmkgfndx
prttqyluibsocyvaezymhgfndx
prtkqylugbsocwvaezjxhgfmdx
prtkqyluibsocwdlezjmhgfnbx
prtkqjluibsocwvaozjhhgfndx
prtcjyluibsocwbaezjmhgfndx
rrtkqyluiblocwvaezjmhgundx
prtkkyluibsocwfaezjmhgfnyx
prtkqyuuibsocwvaezjmhgfogx
prtkyyluvbsocwvaezjmhgfnox
prpkqyluibyocwvaezjmhggndx
pdtkqyluibdocwvaezjmhgfndy
prtklysuibsocwvaezjmhgfnwx
prtkqyluabsouwvaekjmhgfndx
phtkqyluibsocwvaezjmhgfnxt
prtkqyxuibsocwvaezjmhpfnqx
prtkqyluibsodwsaezdmhgfndx
prtkbyluibsohwvaezjmhgfndr
xrtkqylhibsocwvtezjmhgfndx
prtkqyluvysocwvaezbmhgfndx
prtkqieuibsocwvaeojmhgfndx
pctkqyluibsocwvanzjmhgfnux
vrtkqyluibsozwvaezjmhgandx
prtkqyluiusocwvaezjmhmfngx
prbkqyluibsockvaxzjmhgfndx
prtkqyluibsonwvaczjmhgfndi
prtkqyluiblocwvaezjmhgfnau
prtkqyluibsocwvafzuchgfndx
prdkqyluiysocwvaezjmhgfnax
prnkqyouibsocwvaezjmhgfndq
mrtkqgluibsocwvpezjmhgfndx
pvtkqyluibsocwvaczjmhgnndx
trtkqwluibsohwvaezjmhgfndx
prmkqyluibsofwvaezjmhgfrdx
prtyqyluibpdcwvaezjmhgfndx
ertkqylulbsocwvaezjmhgfnax
prtkqyluibsacwvaeijmhgfndf
prtkqyluibyocwvapzjmhgpndx
potkqyluibgocwvaezjmhzfndx
prtkqyluibsocwyaezxmhgfnpx
prtkqkjuibsncwvaezjmhgfndx
prtqqyluibsocwlaezjmhgkndx
prtkxyluibnocwvaezjmhgkndx
prtkqyluiosocwvapzjmxgfndx
prtkqylumbsocwvyezimhgfndx
prukqyluibsocwvyezjmhgindx
prtkqylbibstcwvaezjxhgfndx
pctkqyuuibsocwvaezjuhgfndx
vrtkqyluibsocwvaezjmhgfnll
urtkqyluibsopwvaezjphgfndx
prtkceluibsocwvaepjmhgfndx
prwkxyluibsocwvaezjmhgfnzx
prtkqyluitsocwvaezqzhgfndx
prtkqkauibsorwvaezjmhgfndx
prtkqyluibsocwvaezfmftfndx
prtkiybuibsocwvaezjkhgfndx
prtkzyluibsocwgaezjmvgfndx
prtkqyluibsocwvaezjmhgqnxg
prtkqyluimsocwvauzjwhgfndx
prtkqyluibsacwgaezjmhgfndd
pwtkuyluibsccwvaezjmhgfndx
prtkqyluibsoawvaezjmvgfnlx
prtkqyluabsocwwaezjmhgftdx
patkqylnibsocwvaezjmhgfnox
prtkqyluibsocwlaxzkmhgfndx
pbtkqpluibsfcwvaezjmhgfndx
prtkqyluibsoywsaezjmhgxndx
prtkqyluibfocwvaezjyhgfhdx
pltbqylcibsocwvaezjmhgfndx
prtkdyluiisocwvvezjmhgfndx
prtkqkxuibsokwvaezjmhgfndx
prtkqyluibsoawvaezzmhgfndm
petkqyluibsgcwvaezjmhgfndu
prtkqyluibsoyxvaezjmlgfndx
prtkqyluibxocwvaezgmhnfndx
prtkikluibsocwvwezjmhgfndx
prbkqyluibsocwvaezjhhgfnux
prtkqylufbsxcwvaezjmhgfnfx
prtkqyluibsdcdvaezjmhgxndx
potkiyluibsocwvaezjmhkfndx
prtkqyluiosocsvhezjmhgfndx
prtkqyluibsocqbaezomhgfndx
prtihyluibsocwvaeujmhgfndx
prtuquruibsocwvaezjmhgfndx
prtkqyloibsocwvaeztmhifndx
ertuqyluibsocwvaeajmhgfndx";
