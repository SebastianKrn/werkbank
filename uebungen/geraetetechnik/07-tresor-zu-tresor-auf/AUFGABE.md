# Übung 07 — Tresor zu, Tresor auf

**Ziel:** Du verschlüsselst das Spiegel-Laufwerk mit BitLocker und einem
Kennwort, sperrst es zu, öffnest es wieder — und zeigst, dass Virenschutz und
Firewall laufen.

ca. 35 Minuten · Schwierigkeit 2 von 3 · prüfungsrelevant (LB): ja

Eine gespiegelte Platte schützt gegen Ausfall. Sie schützt nicht davor, dass
jemand die Platte ausbaut und mitnimmt. Dagegen hilft Verschlüsselung. Unsere
VM hat **kein TPM** (den Chip, der den Schlüssel normalerweise verwahrt) — also
übernimmst du diese Rolle mit einem Kennwort.

## Vorher

**Mach einen Snapshot.** Verschlüsselung ist der Schritt, bei dem ein falsches
Kennwort echten Ärger macht.

```powershell
cd C:\werkbank-geraetetechnik
$a = "uebungen\07-tresor-zu-tresor-auf\abgabe"
```

Prüfe, ob `S:` da ist. Nach einem Neustart musst du die Platten wieder anhängen:

```powershell
Mount-DiskImage -ImagePath C:\wb\platten\platte1.vhdx
Mount-DiskImage -ImagePath C:\wb\platten\platte2.vhdx
```

### BitLocker verfügbar machen (nur Windows Server)

Auf Windows Server ist BitLocker ein Feature, das erst installiert werden muss:

```powershell
Get-WindowsFeature BitLocker
```

Steht dort kein `X` bei „Installed“, dann:

```powershell
Install-WindowsFeature BitLocker -IncludeAllSubFeature -Restart
```

Die VM startet neu. **Danach die Platten wieder anhängen** (siehe oben) und die
Variable `$a` erneut setzen.

## Schritte

### 1. Kennwort festlegen

```powershell
$pw = Read-Host "BitLocker-Kennwort" -AsSecureString
```

Du tippst das Kennwort ein, es wird nicht angezeigt. **Mindestens 8 Zeichen.**

**Schreib das Kennwort auf Papier.** Ohne Kennwort und ohne
Wiederherstellungsschlüssel sind die Daten weg — endgültig. Das ist kein
Übungsrisiko, das ist der Sinn der Sache.

### 2. Verschlüsseln

```powershell
Enable-BitLocker -MountPoint S: -PasswordProtector -Password $pw `
    -EncryptionMethod XtsAes256 -UsedSpaceOnly
```

- `-PasswordProtector` ist der Schutz mit Kennwort — genau das Verfahren für
  Systeme ohne TPM.
- `-EncryptionMethod XtsAes256` ist das aktuelle Verfahren: AES mit 256 Bit.
- `-UsedSpaceOnly` verschlüsselt nur den belegten Teil. Das ist schnell und für
  ein neues Laufwerk in Ordnung.

Der Backtick ` am Zeilenende bedeutet: der Befehl geht in der nächsten Zeile
weiter.

Warte, bis die Verschlüsselung fertig ist:

```powershell
Get-BitLockerVolume -MountPoint S: | Format-List VolumeStatus,ProtectionStatus,EncryptionPercentage
```

Fertig ist es bei `VolumeStatus : FullyEncrypted` und `ProtectionStatus : On`.

### 3. Einen Wiederherstellungsschlüssel dazulegen

Ein Kennwort kann man vergessen. Deshalb gibt es einen zweiten Weg hinein:

```powershell
Add-BitLockerKeyProtector -MountPoint S: -RecoveryPasswordProtector
(Get-BitLockerVolume -MountPoint S:).KeyProtector | Out-File C:\wb\bitlocker-schluessel.txt
notepad C:\wb\bitlocker-schluessel.txt
```

Schreib den 48-stelligen Wiederherstellungsschlüssel auf Papier.

**Und jetzt denk nach:** Diese Datei liegt auf `C:` — auf derselben Maschine,
deren Laufwerk sie aufschließt. Wenn der Server brennt oder gestohlen wird,
ist der Schlüssel mit dabei. Trag bei `wo_liegt_der_schluessel` ein, wo so ein
Schlüssel in einer echten Firma hingehört.

### 4. Den Zustand abgeben

```powershell
.\wb erfasse bitlocker 07
notepad "$a\bitlocker.txt"
```

Du suchst zwei Angaben:

- `Schutzstatus: Der Schutz ist aktiviert` (englisch: `Protection Status: Protection On`)
- `Verschlüsselungsmethode: XTS-AES 256` (englisch: `Encryption Method: XTS-AES 256`)

### 5. Tresor zu, Tresor auf

```powershell
Lock-BitLocker -MountPoint S: -ForceDismount
Get-BitLockerVolume -MountPoint S: | Format-List MountPoint,LockStatus,ProtectionStatus |
    Out-File "$a\tresor-zu.txt"
```

Versuche jetzt, das Laufwerk zu öffnen:

```powershell
Get-ChildItem S:\
```

Das muss scheitern. **Der Fehler ist hier das gewünschte Ergebnis.**

Und wieder aufschließen:

```powershell
Unlock-BitLocker -MountPoint S: -Password $pw
Get-ChildItem S:\
```

Wenn `$pw` nicht mehr gilt (neues Fenster), setze es mit `Read-Host` neu — du
tippst dasselbe Kennwort wie vorher.

### 6. Virenschutz und Firewall zeigen

Verschlüsselung ist nur eine Schicht. Zeig die anderen:

```powershell
.\wb erfasse schutz 07
notepad "$a\schutz.txt"
```

In der Datei stehen der Zustand von Microsoft Defender
(`RealTimeProtectionEnabled`) und die drei Firewall-Profile: `Domain`,
`Private`, `Public`.

### 7. Antworten eintragen

```powershell
Copy-Item "uebungen\07-tresor-zu-tresor-auf\material\antworten-vorlage.toml" "$a\antworten.toml"
notepad "$a\antworten.toml"
```

### 8. Aufräumen

Die Schlüsseldatei darf nicht auf der Maschine bleiben:

```powershell
Remove-Item C:\wb\bitlocker-schluessel.txt -Force
Test-Path C:\wb\bitlocker-schluessel.txt | Out-File "$a\aufraeumen.txt"
```

Hast du den Schlüssel auf Papier? Dann ist das hier richtig. Hast du ihn nicht,
hol das jetzt nach, **bevor** du löschst.

**Die Verschlüsselung bleibt an.** Sie wird in Übung 08 wieder abgeschaltet,
wenn der ganze Aufbau abgebaut wird.

### 9. Prüfen

Erst aufräumen, dann prüfen — `wb` schaut auch auf das Aufräumen:

```powershell
.\wb check 07
```

## Falls du Windows 11 nutzt

- BitLocker ist auf Windows 11 **Pro** und **Enterprise** eingebaut, du musst
  nichts installieren. Auf Windows 11 **Home** fehlt BitLocker — dann mach diese
  Übung in einer Server-VM.
- Der Rest ist identisch: `Enable-BitLocker -PasswordProtector` funktioniert für
  Datenlaufwerke ohne TPM genauso.
- Nur für das **Systemlaufwerk** ohne TPM bräuchte man zusätzlich eine
  Gruppenrichtlinie. Wir verschlüsseln ein Datenlaufwerk, also nicht nötig.

## Abgabe

Im Ordner `abgabe`:

- `bitlocker.txt` — Schutz aktiv, AES 256
- `tresor-zu.txt` — der gesperrte Zustand (Bonus)
- `schutz.txt` — Defender und Firewall
- `antworten.toml` — zwei Antworten
- `aufraeumen.txt` — enthält `False`

## KI-Stufe: danach

**Zuerst selbst.** Danach eine gute Frage: „Was ist der Unterschied zwischen
BitLocker mit TPM, mit Kennwort und mit Startschlüssel auf USB?“ Diese drei
Wege muss man auseinanderhalten können — die Frage kommt in Prüfungen gern.

## Reflexion

Der Server ist verschlüsselt. Ein Mitarbeiter kopiert Firmendaten auf einen
privaten USB-Stick. Hat BitLocker hier geholfen?

## Bonus (freiwillig)

1. Der Check `tresor-zu` prüft, ob du den gesperrten Zustand festgehalten hast
   (Schritt 5).
2. Trag bei `wiederherstellungsschluessel_ort` ein, wo dein Schlüssel jetzt
   wirklich liegt (Papier im Ordner? Passwortmanager? Safe?).

## Homelab (freiwillig, für Erfahrene)

Verschlüssle einen USB-Stick mit BitLocker To Go — oder ein Linux-Laufwerk mit
LUKS. Schreib **mindestens 2 Zeilen**: Was war anders, wie wird dort der
Schlüssel verwahrt?

```powershell
notepad "$a\homelab.txt"
```
