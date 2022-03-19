pub fn max_product_(words: Vec<String>) -> i32 {
    let mut res = 0;
    use std::collections::HashSet;
    pub fn ok(a: &String, b: &String) -> bool {
        let a: HashSet<char> = a.chars().collect();
        let b: HashSet<char> = b.chars().collect();
        let intersection = a.intersection(&b);
        intersection.count() == 0
    }

    for i in 0..words.len() {
        for j in (i+1)..words.len() {
            if ok(&words[i], &words[j]) {
                let tmp = words[i].len() as i32 * words[j].len() as i32;
                if tmp > res {
                    res = tmp;
                }
            }
        }
    }
    res
}

//超时，改进：
pub fn max_product(words: Vec<String>) -> i32 {
    let mut res = 0;
    let mask: Vec<i32> = words
        .iter()
        .map(|word| {
            word.chars()
                .fold(0, |acc, c| acc | 1 << (c as u8 - 'a' as u8))
        })
        .collect();
    for i in 0..words.len() {
        for j in (i+1)..words.len() {
            if mask[i] & mask[j] == 0 {
                let tmp = words[i].len() as i32 * words[j].len() as i32;
                if tmp > res {
                    res = tmp;
                }
            }
        }
    }
    res
}


fn main() {
    let words: Vec<String> = vec!["bjciebkhjfogkcndhcigcildkmldjemfkomfnjpbig","fpimalekndkhbpidfeon","gnnbnkbfjhadbhiocfejpkjdcjghcckhohllabghbhjjaeglf","eendocjbnhhdhgcncdkcflaoakkefijnllfbnegehkcdd","ehafeohecdndgajldmglcbgahiacalnoeemkllpkjiblkcbma","ccefhhiihemn","ejjfdipoopakdgf","hlpmeopjjhbokoneadiafimlnmflmgedhdb","gnkhpmodicaknieiaeligpfbgpfjabjghlllppaibhi","bekjcmglmjpojl","mlomfnohbjiocmbkbilmmgkdnfmblngpikmbknjgfmdd","gndn","ddmgoaknalhnlgjd","ajmicdbigiklomkmnjnbldfecidhc","pgmkjbfjaeamaccdocfnccllmpepgmnkacbfjg","caeiggifdcbhkmebohicdpp","jaklijfkelphkdjfpfklmnpgdcdmbmmem","gamlnmfkkbflafkmccea","albofllnlbnkglcepnafckomjbgoimhp","omclhmmkk","cldljfnfplnimdbenbifecmckcabbimafghemlinoh","lkcafi","olgmdelg","cjjngppacgbnlannemmgcfoidodjpnd","mdgbpgpkdamfdnmlhfdllcenjgjeb","mcamoepbcgdaeppkjdkapblpgodoabgomh","cpom","ja","bi","dmnblfmiinlenlnblfmgabipc","dmkooidfjojfdkogmhpefgdend","higopccpcgmefademooccoaepoialclemdnh","dbedcneabcbfdoaelebeoiemgfcffmolkgidkkcepmojplgmlkl","opageimjmpbjpfhopenk","njopafkfonccfadbgncmcchehipoab","dbnphcoickkppinkagdgmbpblonmlficefbmbblcmof","gmkkiohnchgjchiahpeagfcmkocbkcpgghl","meioobnpggeli","ehpghbfmbdhlpfg","gfoadcblpd","gpgdmpdkdoacjaoakolhmjccffmdafed","ialdhfigjndgjgdgnppldgjadcdnkeokbpg","dcpknilcbcb","camdopnkclcdebjhflcmjfkpenccjbdpkjmnekfkoiaai","djahandocccmjagaejhknbcp","idcbohfodjoejjkgeckmjdkhbpk","cgffcodmfelaaenefagnnhdknljlninpeicnmcnajbnaicm","hhjnhpahciiekhoecaippndngkpndgfda","onggmeeicggknobomjk","bjedpkgbjpjoeclofpke","dkgfbedjbfapigae","hmhpifgmhmecbnbkbbepmancfljafpaghhkgbddljdigod","pmmm","bnckjgdkjlncc","ljhplldamfjjdnm","dhhogliijaddffdbgmmmikngbmkanckcbgeehidofnmm","licnbdacgf","hnecehliglhhccfeoboceaod","ppfkfh","ifbmcgjch","gcjchnjjolhkckdjboegohbagbbcokmppjcljajmpnmbagi","moejnhmgaopiocchkimdcmlbgckdilgllippoepnjffhmmp","ad","kcpjkkanobickijjjbaadnbkijoegkpgnkjapeonbldgmfnliki","ildbdlgmekmckhbaogbkbkopikchkahohdebafnmhaf","jdgeglpajjhgbbacankoimaalpkmhppemloppgohpdcdofbj","poljmipkcepljh","chkcfoagabegnjepoinajjbpbodebfjgnjkghjjcapm","cmknmajcmdmecdcphheobcfaechdilloakhob","bjde","cfmmnhnlpffllloepaedcdpnaijillcbmddnlambokgdjpaoel","enmkfninf","cpgfjlceoheghjcmkhfbfkbnhoindnggabnfboeaab","mgjeegpmbckgimgbndlgjibljjhgoimij","pmeaihdabbgnbpnbaoinlonaldn","afideofgbidglafafjinkhgomlaeehafedpgobdbbliod","hbblehjcggfnmcpopalkiimnnafjjbckellk","omffhcdapakikfh","ibnhnhklggfmjkgehfdedkkpjpalkcnoaebcgjaihofnkbphoe","iblmdgipbdilakdgaifekldacopceolehelm","mhkfacjab","ghofaimobcmifeejoc","obkhdpfjlkcpmcciljkpolmflpljdlgjcbkcjmckd","dbnilfgekinojifkdibdodnbohaiblapjddpaninl","ojllgiojblgnkgbnkjcfoiied","mlbkcfgccehgdelkjllloljghjgomojdcmcngcelnkkd","cgkhllconjnn","dpjonioofdbj","dmhbbbgikggodlaggigbbbgmoncjghdm","joacphagfmkblhdhhdkafcfkaohfgoekolk","pngcmlkmialdmoloefnobcgfgb","bklnbaahdlemlbndcflkofaikdpmgcdemjcklceebpiadapb","fmcmiaopeackfleibckoolcicopinkabeolajnefadmc","nbajapjbchklnncakp","bjepknkanjdkbbccgkfgdbnffheklhdhljpgjd","mlncajn","ijpmclbcfdedaemhgbndgdinafjjhnd","mjpaahkielmaniplpgcmddhmofhode","ipgkalchh","foaojbjogbjpfkjk","eaghgabdebbhkbmihbjb","mogkbkabclo","hakknaaejkfcecagmacjl","dngmgbpimklfkilfchchipmlfhciabbk","menjikfbadlhneimnnpcjlibjpaponfipjdhnbipe","lphjjdoljghjlbflpcfjaohafkaolfcljjogfocojjfnnpoab","cbgfa","edkbkfcg","eemjebnbobajnco","lmhohgnhgalakkpnoldonjkhjaeachfgaabd","gdgojhjnabldfmpmoajgoo","hilng","kmmcegjkbhelkgkagpjglmkbijepkfaipokalhdh","anlaokamf","pgcponmomkgmangfcaodhlgealippdggdppoobkghnnmaihp","kkcblbnchlagjo","efmgdiddjdmflmgadiaphhpbmcilpaimmomfodlfigfkjigbnke","ofokkplfhiehopalg","fgkcpacknbdlfjeladiageaiehnimjfmmdkamodhgoieb","ojhfabokagifgfbmhcjamhnlpabhabmjlelcomac","ncpmjlkpdhnpimjldfhdgkf","hmacdopkcgon","afindjcajkdnmhkmohjkedlnodghmfgjmlgpnenlflkfgg"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", max_product(words));
}
