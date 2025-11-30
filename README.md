# Media Stack

Stack Docker completo para gerenciamento de mídia com Traefik reverse proxy e descoberta automática de serviços via labels.

## Serviços Incluídos

| Serviço | Descrição | URL Local |
|---------|-----------|-----------|
| **Traefik** | Reverse proxy com auto-discovery | https://traefik.home.local |
| **Homarr** | Dashboard/Homepage | https://home.local |
| **Jellyfin** | Media server (GPU) | https://jellyfin.home.local |
| **Plex** | Media server | https://plex.home.local |
| **Jellyseerr** | Request management | https://jellyseerr.home.local |
| **Sonarr** | TV Shows manager | https://sonarr.home.local |
| **Radarr** | Movies manager | https://radarr.home.local |
| **Bazarr** | Subtitles manager | https://bazarr.home.local |
| **Prowlarr** | Indexer manager | https://prowlarr.home.local |
| **qBittorrent** | Torrent client | https://qbittorrent.home.local |
| **SABnzbd** | Usenet client | https://sabnzbd.home.local |
| **Tdarr** | Media transcoding (GPU) | https://tdarr.home.local |
| **Unpackerr** | Auto extract downloads | https://unpackerr.home.local |
| **Portainer** | Docker management | https://portainer.home.local |
| **AdGuard Home** | DNS/Ad blocker | https://adguard.home.local |
| **Home Assistant** | Home automation | http://192.168.1.250:8123 |
| **Tandoor** | Recipe manager | https://recipes.home.local |
| **N8N** | Workflow automation | https://n8n.home.local |
| **Vert/Vertd** | File conversion (GPU) | https://vert.home.local |
| **FlareSolverr** | Cloudflare bypass | https://flaresolverr.home.local |

## Requisitos

- Docker e Docker Compose
- NVIDIA Container Toolkit (para GPU)
- OpenSSL (para gerar certificados)

## Instalação

### 1. Clonar repositório

```bash
git clone <repo-url>
cd Media-Stack
```

### 2. Configurar variáveis de ambiente

Edite o arquivo `.env` com suas configurações:

```bash
# Network Configuration
DOCKER_SUBNET=172.28.10.0/24
DOCKER_GATEWAY=172.28.10.1
LOCAL_DOCKER_IP=192.168.1.250

# User/Group Configuration
PUID=1000
PGID=1000
UMASK=022

# Timezone
TIMEZONE=America/Sao_Paulo

# Theme Park
TP_THEME=dracula

# Folder Paths
FOLDER_FOR_MEDIA=/mnt/media
FOLDER_FOR_DATA=/mnt/configs

# Domain Configuration
LOCAL_DOMAIN=home.local
```

### 3. Gerar certificados SSL auto-assinados

```bash
chmod +x generate-certs.sh
./generate-certs.sh
```

O script criará os certificados em `./traefik/certs/`.

### 4. Configurar DNS local (AdGuard)

Após iniciar o stack, configure o AdGuard Home para resolver os domínios locais:

1. Acesse http://192.168.1.250:3000 (setup inicial)
2. Vá em **Filters** → **DNS rewrites**
3. Adicione: `*.home.local` → `192.168.1.250`

### 5. Iniciar o stack

```bash
docker compose up -d
```

## Estrutura de Pastas

```
FOLDER_FOR_MEDIA (/mnt/media)
├── media/
│   ├── movies/      # Radarr
│   ├── tv/          # Sonarr
│   ├── music/       # Lidarr
│   └── ...
├── torrents/
│   ├── movies/
│   ├── tv/
│   └── ...
└── usenet/
    ├── movies/
    ├── tv/
    └── ...

FOLDER_FOR_DATA (/mnt/configs)
├── traefik/
├── jellyfin/
├── sonarr/
├── radarr/
├── ...
```

## Arquivos de Configuração

| Arquivo | Descrição |
|---------|-----------|
| `docker-compose.yaml` | Configuração principal dos containers |
| `.env` | Variáveis de ambiente |
| `traefik/traefik.yml` | Configuração estática do Traefik |
| `traefik/dynamic.yml` | Configuração dinâmica (certificados) |
| `generate-certs.sh` | Script para gerar certificados |

## Traefik Auto-Discovery

Todos os serviços são automaticamente descobertos pelo Traefik através de labels no docker-compose:

```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.SERVICO.rule=Host(`SERVICO.${LOCAL_DOMAIN}`)"
  - "traefik.http.routers.SERVICO.entrypoints=websecure"
  - "traefik.http.routers.SERVICO.tls=true"
  - "traefik.http.services.SERVICO.loadbalancer.server.port=PORTA"
```

Para adicionar um novo serviço, basta incluir as labels apropriadas.

## GPU (NVIDIA)

Os seguintes serviços utilizam GPU para transcoding:
- Jellyfin
- Tdarr
- Vertd

Requisitos:
- NVIDIA Driver instalado
- NVIDIA Container Toolkit

## Portas Expostas

Algumas portas são expostas diretamente (além do Traefik):

| Porta | Serviço | Motivo |
|-------|---------|--------|
| 80 | Traefik | HTTP (redirect para HTTPS) |
| 443 | Traefik | HTTPS |
| 53 | AdGuard | DNS |
| 3000 | AdGuard | Setup inicial |
| 32400 | Plex | Direct Play |
| 6887 | qBittorrent | Torrent |
| 8266 | Tdarr | Node communication |

## Watchtower

Todos os containers estão configurados com label para atualização automática via Watchtower:

```yaml
labels:
  - "com.centurylinklabs.watchtower.enable=true"
```

O Watchtower verifica atualizações a cada 24 horas.

## Confiar no Certificado SSL

Para evitar avisos de segurança no navegador:

**Linux:**
```bash
sudo cp ./traefik/certs/local.crt /usr/local/share/ca-certificates/home-local.crt
sudo update-ca-certificates
```

**macOS:**
```bash
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain ./traefik/certs/local.crt
```

**Windows:**
1. Abra `./traefik/certs/local.crt`
2. Clique em "Instalar Certificado"
3. Selecione "Máquina Local"
4. Selecione "Colocar todos os certificados no repositório a seguir"
5. Clique em "Procurar" e selecione "Autoridades de Certificação Raiz Confiáveis"
6. Conclua a instalação

## Comandos Úteis

```bash
# Iniciar todos os containers
docker compose up -d

# Parar todos os containers
docker compose down

# Ver logs de um container
docker compose logs -f <container>

# Reiniciar um container específico
docker compose restart <container>

# Atualizar imagens
docker compose pull
docker compose up -d

# Verificar status
docker compose ps
```

## Troubleshooting

### Erro NVIDIA Runtime
Se `vertd`, `jellyfin` ou `tdarr` falharem com erro NVIDIA:
1. Verifique se o NVIDIA Container Toolkit está instalado
2. Verifique se o driver NVIDIA está funcionando: `nvidia-smi`
3. Remova a seção `deploy.resources.reservations.devices` do serviço

### Certificado não confiável
Execute o script de geração de certificados e importe no sistema operacional.

### DNS não resolve
Verifique se o AdGuard está configurado e se seu dispositivo está usando ele como DNS.

## Licença

Este projeto é fornecido como está, sem garantias.
