use std::fs;
use crate::voicebank::Voicebank;
use anyhow::Result;

pub const ASSETS_DIR: &str = "voicebank_assets";
pub const README_PATH: &str = "readme.md";

fn parse_config(content: &str, folder_name: String) -> Voicebank {
    let mut vb = Voicebank {
        folder_name,
        ..Voicebank::new()
    };

    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let value = value.trim().to_string();
            match key.trim() {
                "nome_do_vb" => vb.name = value,
                "link_do_vb" => vb.download_link = value,
                "design_do_vb" => vb.image_path = value,
                "criador_do_vb" => vb.creator = value,
                "metodo_do_vb" => vb.bank_type = value,
                "idioma_do_vb" => vb.language = value,
                _ => {}
            }
        }
    }
    vb
}

fn serialize_config(vb: &Voicebank) -> String {
    format!(
        "nome_do_vb: {}\nlink_do_vb: {}\ndesign_do_vb: {}\ncriador_do_vb: {}\nmetodo_do_vb: {}\nidioma_do_vb: {}",
        vb.name, vb.download_link, vb.image_path, vb.creator, vb.bank_type, vb.language
    )
}

pub fn load_voicebanks() -> Vec<Voicebank> {
    let mut vbs = Vec::new();
    if let Ok(entries) = fs::read_dir(ASSETS_DIR) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                let config_path = entry.path().join("config.txt");
                if let Ok(content) = fs::read_to_string(config_path) {
                    vbs.push(parse_config(&content, folder_name));
                } else {
                    // Se não tiver config, cria um vazio com o nome da pasta
                    vbs.push(Voicebank {
                        name: folder_name.clone(),
                        folder_name,
                        ..Voicebank::new()
                    });
                }
            }
        }
    }
    vbs
}

pub fn save_voicebanks(vbs: &[Voicebank]) -> Result<()> {
    for vb in vbs {
        let folder_path = std::path::Path::new(ASSETS_DIR).join(&vb.folder_name);
        if !folder_path.exists() {
            fs::create_dir_all(&folder_path)?;
        }
        let config_path = folder_path.join("config.txt");
        let content = serialize_config(vb);
        fs::write(config_path, content)?;
    }
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

pub fn sync_with_github() -> Result<String> {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(["pull", "origin", "main"])
        .output()?;
        
    if output.status.success() {
        Ok("Sincronização concluída com sucesso!".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        Err(anyhow::anyhow!("Erro na sincronização: {}", error))
    }
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
