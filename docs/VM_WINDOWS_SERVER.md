# Test-VM aufbauen — Windows Server 2022 auf Linux

**Wofür:** Du brauchst eine frische Windows-Server-2022-Maschine, um
`docs/TESTPROTOKOLL.md` durchzuspielen. Diese Anleitung baut sie von null auf.

**Warum genau so:** Die Leistungsbeurteilung (LB) läuft auf einer **QEMU-VM mit
UEFI und ohne TPM**. Wir bauen dieselbe Umgebung. Ein Test auf etwas anderem
misst etwas anderes.

**Aufwand:** ca. 90 Minuten, davon ca. 50 Minuten reines Warten (Download,
Windows-Installation). Du kannst in der Wartezeit die Entscheidungen in
`docs/M3B_ANLEITUNG.md` treffen.

**Du brauchst:** einen Linux-Rechner, ~80 GB freien Plattenplatz, mindestens
8 GB RAM, Internet.

---

## Schritt 0 — Virtualisierung im BIOS einschalten

**Auf deinem Arch-Notebook ist sie aktuell AUS.** Nachgeprüft am 2026-08-02:
`/proc/cpuinfo` enthält kein `vmx`, und `/dev/kvm` existiert nicht. Der
Prozessor (Intel i7-1165G7) kann es — es ist nur in der Firmware abgeschaltet.

Ohne diesen Schritt läuft Windows in reiner Software-Emulation. Das ist
10–20-mal langsamer; die Windows-Installation dauert dann Stunden statt Minuten.

**Prüfen (vorher):**

```bash
grep -c vmx /proc/cpuinfo        # 0 = aus, >0 = an
```

**Einschalten:**

1. Rechner neu starten.
2. Während des Startbildschirms die Setup-Taste drücken. Welche das ist, steht
   kurz am Bildschirm — üblich sind **F2**, **F10**, **Entf** oder **Esc**.
   Wenn du sie verpasst: neu starten und mehrmals hintereinander drücken.
3. Im Setup suchen unter **Advanced**, **Security** oder **CPU Configuration**
   nach einem Eintrag mit einem dieser Namen:
   - `Intel (VMX) Virtualization Technology`
   - `Intel Virtualization Technology`
   - `VT-x`
4. Auf **Enabled** stellen.
5. Speichern und neu starten (meist **F10**).

**Prüfen (nachher):**

```bash
grep -c vmx /proc/cpuinfo        # muss jetzt > 0 sein
```

Steht dort weiterhin `0`, hast du die falsche Einstellung erwischt. Es gibt in
manchen Firmwares zwei getrennte Schalter (`VT-x` und `VT-d`) — gemeint ist
`VT-x`.

---

## Schritt 1 — Virtualisierung installieren

### Arch (dein Notebook)

```bash
sudo pacman -S --needed qemu-full libvirt virt-manager virt-viewer dnsmasq edk2-ovmf
sudo systemctl enable --now libvirtd.socket virtlogd.socket
sudo usermod -aG libvirt "$USER"
```

### Fedora (dein zweiter Rechner)

```bash
sudo dnf install -y qemu-kvm libvirt virt-manager virt-install virt-viewer edk2-ovmf
sudo systemctl enable --now libvirtd
sudo usermod -aG libvirt "$USER"
```

**Danach abmelden und wieder anmelden** — sonst greift die Gruppenmitgliedschaft
nicht. (Kurzfristig geht auch `newgrp libvirt` im aktuellen Terminal.)

**Prüfen:**

```bash
ls -l /dev/kvm                   # muss existieren
virsh list --all                 # darf leer sein, aber darf NICHT nach root fragen
virsh net-list --all             # "default" muss da sein
```

Ist `default` nicht aktiv:

```bash
sudo virsh net-start default
sudo virsh net-autostart default
```

Meldet `virsh list` einen Verbindungsfehler, läuft dein libvirt im modularen
Modus. Dann stattdessen:

```bash
sudo systemctl enable --now virtqemud.socket virtnetworkd.socket virtstoraged.socket
```

---

## Schritt 2 — Die ISO herunterladen

### Windows Server 2022 (Evaluierung, kostenlos, 180 Tage)

<https://www.microsoft.com/de-de/evalcenter/evaluate-windows-server-2022>

Du musst dich mit einer Mailadresse registrieren. Dann:

| Auswahl | Nimm | Warum |
|---|---|---|
| Format | **ISO (64-bit)** | nicht VHD, nicht Azure |
| Sprache | **Deutsch** | Teil B und C des Protokolls prüfen Umlaute und die deutsche Konsole. Auf einem englischen Windows misst du das falsche System. |
| Edition | Standard oder Datacenter | egal, nimm Standard |

Größe ca. 5 GB. Leg sie nach `~/isos/`.

> **Hinweis zur Evaluierung:** Sie läuft 180 Tage. Microsoft verlangt eine
> Aktivierung über Internet innerhalb der ersten 10 Tage, sonst fährt die
> Maschine selbstständig herunter. Für einen Testtag ist das egal — wenn du die
> VM länger behältst, lass sie einmal online.

### virtio-Treiber (optional, nur bei Geschwindigkeitsproblemen)

Nicht nötig für den Standardweg unten. Wenn die VM zu langsam ist:
<https://fedorapeople.org/groups/virt/virtio-win/direct-downloads/stable-virtio/virtio-win.iso>
(ca. 750 MB, geprüft am 2026-08-02).

---

## Schritt 3 — Die VM anlegen

Wir nehmen **SATA-Platte und e1000e-Netzwerkkarte**. Für beide bringt Windows
die Treiber mit — du musst während der Installation nichts nachladen. Das ist
etwas langsamer als virtio und spart dir die häufigste Fehlerquelle.

### Weg A — grafisch (empfohlen)

```bash
virt-manager
```

**Datei → Neue virtuelle Maschine**, dann:

| Schritt | Einstellung |
|---|---|
| 1. Installationsart | Lokales Installationsmedium (ISO) |
| 2. ISO auswählen | deine Windows-Server-ISO aus `~/isos/` |
| Betriebssystem | `Microsoft Windows Server 2022` (tippen, bis es vorgeschlagen wird) |
| 3. Speicher | **4096 MB** |
| 3. CPUs | **2** |
| 4. Festplatte | **60 GB**, qcow2 |
| 5. Name | `wb-test` |
| 5. | **„Konfiguration bearbeiten vor der Installation" ankreuzen** |

Im Konfigurationsfenster, das jetzt aufgeht, drei Dinge:

1. **Übersicht → Firmware:** auf **UEFI** stellen (`OVMF_CODE.fd`).
   Das ist die Umgebung der LB.
2. **KEIN TPM hinzufügen.** Die LB-VM hat keins, und Übung 07 baut genau darauf
   auf („unsere VM hat kein TPM, also übernimmst du diese Rolle mit einem
   Kennwort"). Ein TPM würde die Übung verfälschen.
3. **NIC:** Gerätemodell auf **e1000e**.

Dann **Installation beginnen**.

### Weg B — auf der Kommandozeile

```bash
mkdir -p ~/vms
virt-install \
  --name wb-test \
  --osinfo win2k22 \
  --memory 4096 \
  --vcpus 2 \
  --cpu host-passthrough \
  --boot uefi \
  --disk path="$HOME/vms/wb-test.qcow2",size=60,format=qcow2,bus=sata \
  --cdrom "$HOME/isos/windows-server-2022-de.iso" \
  --network network=default,model=e1000e \
  --graphics spice
```

Meldet er `Unknown OS name 'win2k22'`, zeig dir die verfügbaren Namen an und
nimm den passendsten:

```bash
osinfo-query os | grep -i "win2k2"
```

---

## Schritt 4 — Windows installieren

1. **„Press any key to boot from CD"** — drück innerhalb von fünf Sekunden eine
   Taste. Verpasst? VM neu starten (Fenster → Virtuelle Maschine → Neu starten).
2. Sprache/Tastatur: **Deutsch**.
3. Edition wählen: **„Windows Server 2022 Standard (Desktop-Darstellung)"**.

   > **Das ist die wichtigste Auswahl der ganzen Installation.** Ohne
   > „Desktop-Darstellung" bekommst du Server Core: keine Oberfläche, kein
   > Explorer, kein Edge, kein Notepad. Das Testprotokoll und alle acht Übungen
   > setzen die Oberfläche voraus.

4. Lizenzbedingungen annehmen.
5. **„Benutzerdefiniert: nur Windows installieren"**.
6. Die 60-GB-Platte auswählen → **Weiter**. Nicht partitionieren, Windows macht
   das selbst.
7. Warten (10–20 Minuten, mehrere Neustarts).
8. Kennwort für **Administrator** setzen. **Schreib es auf.** Es gibt keinen
   Weg zurück.

Fertig. Du bist als `Administrator` angemeldet — das ist automatisch ein
frisches Konto ohne deine Entwickler-Werkzeuge. Genau das verlangt das
Protokoll.

---

## Schritt 5 — Die fünf Dinge nach der Installation

### 5.1 Internet Explorer Enhanced Security abschalten

**Ohne diesen Schritt kannst du in der VM nichts herunterladen.** Windows
Server sperrt jede Webseite, die nicht ausdrücklich freigegeben ist. Die Sperre
gilt auf Server 2022 auch für Microsoft Edge.

1. **Server-Manager** öffnet sich beim Anmelden von selbst (sonst: Startmenü →
   Server-Manager).
2. Links **Lokaler Server** anklicken.
3. In der Eigenschaftsliste **„Verstärkte Sicherheitskonfiguration für IE"**
   suchen — daneben steht **Ein**. Draufklicken.
4. Beide Schalter auf **Aus** (Administratoren *und* Benutzer) → **OK**.

### 5.2 Prüfen, dass Edge da ist und Internet geht

```powershell
Test-NetConnection github.com -Port 443
```

`TcpTestSucceeded : True` = Netzwerk in Ordnung. Dann Edge öffnen (Startmenü →
Microsoft Edge) und `github.com` aufrufen.

### 5.3 PowerShell als Administrator

Rechtsklick auf das Start-Symbol → **Windows PowerShell (Administrator)**.
Alles im Protokoll läuft in diesem Fenster.

### 5.4 Die Rücksetz-Kopie anlegen

Das ist dein Snapshot. **Mach das, bevor du irgendetwas anderes tust.**

VM herunterfahren (Windows → Start → Herunterfahren), dann auf dem Linux-Host:

```bash
cp ~/vms/wb-test.qcow2 ~/vms/wb-test-FRISCH.qcow2
```

Zurücksetzen geht später so (VM muss aus sein):

```bash
cp ~/vms/wb-test-FRISCH.qcow2 ~/vms/wb-test.qcow2
```

Dauert ein paar Sekunden und funktioniert immer.

> **Warum nicht die Snapshot-Funktion von virt-manager?** Kannst du probieren
> (Kamerasymbol im VM-Fenster). Bei UEFI-Maschinen weigern sich manche
> libvirt-Versionen mit einer Meldung über „pflash". Die Dateikopie hat dieses
> Problem nicht. Lernende im Unterricht nutzen die Snapshot-Funktion ihres
> eigenen Hypervisors — das ist ein anderer Weg und hier nicht dein Problem.

### 5.5 Zweite Kopie nach Teil B

Die Übungen 03–07 bauen aufeinander auf. Wenn Teil B des Protokolls sauber
durch ist, mach eine zweite Kopie (`wb-test-NACH-B.qcow2`). Dann musst du bei
einem Fehler in Übung 05 nicht wieder bei Windows-Installation anfangen.

---

## Schritt 6 — Das ZIP in die VM holen

**In der VM, mit Edge**, auf die Release-Seite gehen:

<https://github.com/SebastianKrn/werkbank/releases/tag/v0.1.0-rc2>

Zwei Dateien herunterladen:

- `werkbank-geraetetechnik-v0.1.0-rc2.zip`
- `SHA256SUMS.txt`

> **Nicht über einen geteilten Ordner, nicht über die Zwischenablage, nicht per
> USB kopieren.** Der Download durch den Browser ist es, der die
> „Mark-of-the-Web"-Markierung setzt. Genau die prüft Teil A des Protokolls.
> Kopierst du die Datei hinein, testest du die Hälfte von Teil A nicht.

**Prüfsumme kontrollieren** (PowerShell, im Download-Ordner):

```powershell
cd $HOME\Downloads
$datei    = "werkbank-geraetetechnik-v0.1.0-rc2.zip"
$erwartet = ((Select-String -Path .\SHA256SUMS.txt -Pattern $datei).Line -split '\s+')[0]
$ist      = (Get-FileHash $datei -Algorithm SHA256).Hash
if ($ist -ieq $erwartet) { "OK - Prüfsumme stimmt" }
else { "FALSCH - nicht entpacken!`n  erwartet: $erwartet`n  ist:      $ist" }
```

`Get-FileHash` liefert Großbuchstaben, `SHA256SUMS.txt` Kleinbuchstaben. Der
Vergleich oben (`-ieq`) ignoriert das. Von Hand vergleichen führt zu falschem
Alarm.

Steht dort **FALSCH**: nicht entpacken, Download wiederholen. Bleibt es falsch,
ist das ein Fund für Teil G des Protokolls.

**Jetzt weiter mit `docs/TESTPROTOKOLL.md`, Teil A.**

---

## Anhang — Der Contabo-Windows-Server

Du hast einen Windows-Server bei Contabo gemietet und erreichst ihn per
Remotedesktop von Fedora aus.

### „Wie gebe ich ihm Internet?"

**Er hat schon Internet.** Ein Contabo-VPS hat eine öffentliche IP-Adresse —
sonst könntest du dich gar nicht per Remotedesktop verbinden. Was fehlt, ist
fast sicher nicht das Netzwerk, sondern die Browser-Sperre aus Schritt 5.1.

Prüf es in einer PowerShell auf dem Server:

```powershell
Test-NetConnection github.com -Port 443
```

| Ergebnis | Bedeutung | Was tun |
|---|---|---|
| `TcpTestSucceeded : True` | Internet ist da, nur der Browser blockt | Schritt 5.1 durchführen — „Verstärkte Sicherheitskonfiguration für IE" auf **Aus** |
| `TcpTestSucceeded : False` | echtes Netzwerkproblem | `Get-NetIPConfiguration` — hat die Karte IP-Adresse, Gateway und DNS? Wenn nicht: im Contabo-Kundenpanel die Netzwerkkonfiguration prüfen |

Mit QEMU hat das nichts zu tun. Contabo betreibt zwar KVM/QEMU im Hintergrund,
aber das Netzwerk deiner VM ist dort schon fertig konfiguriert.

### Kann ich das Protokoll auf dem Contabo-Server laufen lassen?

**Besser nicht.** Vier Gründe, in der Reihenfolge ihres Gewichts:

1. **Kein schnelles Zurücksetzen.** Die Übungen 03–07 bauen einen Spiegel auf
   und zerstören ihn wieder. Ohne eine Kopie, die du in 20 Sekunden
   zurückspielst, kostet dich jeder Fehler eine Stunde.
2. **Übung 06 und 07 sind für eine Maschine gefährlich, die du nur über das
   Netz erreichst.** Du simulierst einen Plattenausfall und verschlüsselst ein
   Laufwerk. Geht dabei etwas schief, sitzt du nicht davor.
3. **Der Offline-Betrieb bliebe dort für immer unprüfbar.** Das Protokoll hat
   heute keinen Schritt, der das Netzwerk trennt — „läuft ohne Internet" steht
   bisher nur in SPEC §2 („Security constraints") und ADR 0001. Auf der lokalen
   VM kannst du die Netzwerkkarte jederzeit abhängen und es nachholen. Auf einer
   Maschine, die du ausschließlich über das Netz bedienst, kannst du das nie.
4. **Die LB läuft auf einer lokalen QEMU-VM.** Der Contabo-Server ist eine
   andere Umgebung.

**Wofür der Contabo-Server gut ist:** als Ersatz, falls VT-x auf deinem
Notebook nicht aktivierbar ist, und als zweite Meinung — wenn ein Fund nur auf
einer Maschine auftritt, weißt du mehr.
