# Zadanie1
Dockerfile w poprzedniej wersji nie działał i nie dało się zbudować obrazu, w tej wersji zostało to naprawione

- budowanie obrazu:
```bash
docker build --no-cache -t pogoda:latest .
```

- uruchomienie obrazu:
```bash
docker run -p 8080:8080 pogoda:latest.
```


# Zadanie2
---
### 1. Checkout kodu źródłowego  
Na samym początku łańcuchu wykorzystywany jest krok **"Checkout repository"** z akcją `actions/checkout@v4`, który klonuje repozytorium z kodem źródłowym. Dzięki temu wszystkie dalsze operacje (budowanie obrazu, testy) operują na aktualnej wersji aplikacji.

---

### 2. Konfiguracja emulacji dla multiarchitektury  
Aby możliwe było budowanie obrazu wspierającego różne architektury, wykorzystywany jest krok **"Setup QEMU"** przy użyciu akcji `docker/setup-qemu-action@v3`. Emulacja QEMU umożliwia symulację środowiska dla architektur takich jak linux/arm64, nawet jeśli maszyna budująca natywnie tej architektury nie obsługuje.

---

### 3. Konfiguracja Docker Buildx  
Kolejnym krokiem jest konfiguracja narzędzia Docker Buildx, realizowana przez akcję `docker/setup-buildx-action@v3`. Buildx umożliwia tworzenie buildów wieloarchitekturowych, co jest kluczowe do spełnienia wymogów – obraz musi wspierać architektury linux/amd64 oraz linux/arm64.

---

### 4. Generowanie metadanych i definicja tagów obrazu  
W kroku **"Docker metadata definitions"** użyta została akcja `docker/metadata-action@v5`, która generuje metadane odpowiadające za tagowanie obrazu. Przyjęto dwa sposoby tagowania:  
- **Tag SHA** – wygenerowany na podstawie identyfikatora commita, z prefiksem `sha-` i formatowaniem skróconym. Ma on wysoką priorytetowość (priority=100) i gwarantuje unikalność obrazu względem konkretnego stanu kodu.  
- **Tag semver** – oparty na wersjonowaniu semantycznym (pattern `{{version}}`) o niższym priorytecie (priority=200). Pozwala on na przypisywanie czytelnych wersji finalnych obrazu.  

To podejście umożliwia łatwe śledzenie, która wersja obrazu odpowiada konkretnemu commitowi, oraz wspiera identyfikację stabilnych wersji wydania.

---

### 5. Autoryzacja do rejestrów – DockerHub i GHCR  
Aby korzystać z mechanizmu cache'owania i móc wypychać obrazy, pipeline loguje się jednocześnie do dwóch miejsc:
- **DockerHub** – logowanie realizowane jest przez akcję `docker/login-action@v3`. Dzięki temu możliwe jest pobieranie i wysyłanie danych cache. Cache jest przechowywany w dedykowanym publicznym repozytorium autora na DockerHub (np. `${{ vars.DOCKERHUB_USERNAME }}/pogoda:cache`).
- **GitHub Container Registry (GHCR)** – logowanie do GHCR umożliwia ostateczne wypchnięcie przetestowanego obrazu. Tutaj wykorzystano takie same narzędzie do logowania, a dane uwierzytelniające pobierane są odpowiednio z kontekstu GitHub (actor, secret).

---

### 6. Budowanie obrazu – wsparcie dla wielu architektur oraz użycie cache  
Krok **"Build Docker image (candidate tag)"** realizowany przez akcję `docker/build-push-action@v5` odpowiada za budowę obrazu z wykorzystaniem Dockerfile oraz zbudowanie obrazu pod kątem dwóch architektur: linux/amd64 i linux/arm64.  
- Obraz oznaczony jest tymczasowym tagiem **candidate**, co pozwala na przeprowadzenie dalszych testów bez ingerencji w finalne tagi.  
- Wykorzystywane są mechanizmy cache'owania – dzięki opcji `cache-from` pobierany jest poprzedni stan cache, a po zakończeniu budowania nowy stan jest wysyłany przy użyciu `cache-to` (tryb `max`) do dedykowanego repozytorium na DockerHub. To rozwiązanie przyspiesza kolejne buildy oraz zmniejsza koszty przesyłu danych.

---

### 7. Testowanie obrazu – skanowanie pod kątem luk CVE  
Następnie obraz oznaczony jako candidate jest poddawany testom bezpieczeństwa przy użyciu Trivy (akcja `aquasecurity/trivy-action@master`).  
- Skanowanie analizuje obraz pod kątem luk o krytycznym i wysokim poziomie zagrożenia (CRITICAL, HIGH).  
- Konfiguracja `exit-code: '1'` powoduje przerwanie dalszych kroków, jeśli zostaną wykryte nieakceptowalne zagrożenia. Dzięki temu do finalnego repozytorium trafiają tylko obrazy spełniające normy bezpieczeństwa.

---

### 8. Retagowanie i wypchnięcie finalnych obrazów  
Po pozytywnym wyniku testów CVE następuje krok **"Apply final tags and push image"**. W tym etapie:  
- Obraz o tagu candidate jest pobierany na maszynę buildującą.  
- Skrypt iteracyjnie pobiera kolejne tagi wygenerowane przez akcję `docker/metadata-action` i dokonuje retagowania obrazu.  
- Każda z nowych etykietowana wersja zostaje wypchnięta do GitHub Container Registry (GHCR), co umożliwia jej późniejsze wykorzystanie przez użytkowników.

---

### 9. Sprzątanie – usuwanie tymczasowego tagu  
Na zakończenie pipeline wykonuje opcjonalny krok **"Cleanup candidate tag"**, który usuwa obraz oznaczony jako candidate z lokalnego repozytorium. Pozwala to zachować porządek i zmniejszyć użycie pamięci na serwerze buildującym.

---

