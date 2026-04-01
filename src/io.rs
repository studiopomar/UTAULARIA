use std::fs;
use crate::voicebank::Voicebank;
use anyhow::Result;

pub const DATA_PATH: &str = "voicebanks.yaml";
pub const README_PATH: &str = "readme.md";

pub fn load_voicebanks() -> Vec<Voicebank> {
    if let Ok(content) = fs::read_to_string(DATA_PATH) {
        if let Ok(vbs) = serde_yaml::from_str(&content) {
            return vbs;
        }
    }
    Vec::new()
}

pub fn save_voicebanks(vbs: &[Voicebank]) -> Result<()> {
    let content = serde_yaml::to_string(vbs)?;
    fs::write(DATA_PATH, content)?;
    Ok(())
}

pub fn update_readme(vbs: &[Voicebank]) -> Result<()> {
    let readme_content = fs::read_to_string(README_PATH)?;
    
    let _marker_start = "| Nome | Criador | Tipo | Idioma | Download |";
    let marker_separator = "|------|---------|------|--------|----------|";
    
    if let Some(start_pos) = readme_content.find(marker_separator) {
        let before = &readme_content[..start_pos + marker_separator.len()];
        
        // Find where the table ends (first empty line or next header)
        let rest = &readme_content[start_pos + marker_separator.len()..];
        let end_pos = rest.find("\n\n").unwrap_or(rest.len());
        let after = &rest[end_pos..];

        let mut table_rows = String::new();
        for vb in vbs {
            table_rows.push_str(&format!(
                "\n| *{}* | *{}* | *{}* | *{}* | *[Download]({})* |",
                vb.name, vb.creator, vb.bank_type, vb.language, vb.download_link
            ));
        }

        let new_content = format!("{}{}{}", before, table_rows, after);
        fs::write(README_PATH, new_content)?;
    }

    Ok(())
}

pub fn validate_voicebank(path: &std::path::Path) -> Result<String> {
    if !path.is_dir() {
        return Ok("Caminho não é um diretório.".to_string());
    }

    let entries: Vec<_> = fs::read_dir(path)?.filter_map(|e| e.ok()).collect();
    let has_oto = entries.iter().any(|e| e.file_name() == "oto.ini");
    let has_wav = entries.iter().any(|e| e.path().extension().map_or(false, |ext| ext == "wav"));
    let has_readme = entries.iter().any(|e| e.file_name() == "readme.txt" || e.file_name() == "characther.txt");

    let mut report = String::new();
    if has_oto && has_wav {
        report.push_str("[OK] Estrutura técnica básica OK (oto.ini + wavs encontrados).");
    } else {
        if !has_oto { report.push_str("[ERRO] Falta o arquivo oto.ini.\n"); }
        if !has_wav { report.push_str("[ERRO] Nenhum arquivo .wav encontrado.\n"); }
    }

    if has_readme {
        report.push_str("\n[OK] Documentação encontrada.");
    } else {
        report.push_str("\n[AVISO] Recomendado adicionar readme.txt ou characther.txt.");
    }

    Ok(report)
}
