#[cfg(test)]
mod parse_calc_tests {
    #[test]
    fn parse_sample1() {
        let content = vec!["masterpiece,best quality,official art,extremely,detailed cg 8k wallpaper,
        (flying petals),perfect female body,curvy,milf, huge breasts,nsfw,Fully exposed, nipple exposed, breast exposed, pudenda exposed, nipple obvious, pudenda obvious, breast beautiful,full body,
        (detailed ice),crystalstexture skin,cold expression,white hair,long hair,messy hair,blue eye, looking at viewer, extremely delicate and beautiful,water,((beauty detailed eye)),highly detailed,cinematic lighting,((beautiful face)), fine water surface,(original figure painting),ultra-detailed,incredibly detailed,(an extremely delicate and beautiful), beautiful detailed eyes. (best quality),shelf bra,pointy breasts,
        ,breasts out,nipple cutout,",
        "1girl,(best quality,masterpiece),ultra detailed,1girl, black hair, blush, solo, skirt, socks, brown eyes, child, sleeveless, twintails, looking at viewer, short hair <lora:shizuka:1>",
        "nsfw,2girl,(worst quality,low quality:1.4),(bad_prompt_version2:0.8), (monochrome:1.1),"
        ];

        assert!(content.len() > 0);
    }
}
