# Tutoriel d'intégration Rust-Python avec PyO3

Ce projet démontre comment intégrer du code Rust dans Python en utilisant
**PyO3** et **Maturin**. Il implémente un module Python `piledger` qui expose
des classes Rust pour la gestion de comptes comptables et la lecture de
fichiers CSV.


## 🚀 Installation et configuration

### 1. Créer un environnement virtuel Python

```bash
# Créer l'environnement virtuel
python -m venv .venv

# Activer l'environnement (Windows PowerShell)
.\.venv\Scripts\Activate.ps1

# Activer l'environnement (Linux/macOS)
source .venv/bin/activate
```

### 2. Installer Maturin

```bash
pip install maturin
```

### 2.1 Initialiser un nouveau projet Maturin (si vous partez de zéro)

Cette étape est nécessaire uniquement si vous créez un nouveau projet.

```bash
maturin init --bindings pyo3 --name piledger
```


### 3. Modifier le code Rust (optionnel)

Le fichier principal se trouve dans `src/lib.rs`. Ce fichier contient :
- La classe `Account` : représente un compte avec nom, type et numéro
- La classe `AccountReader` : lit des comptes depuis un fichier CSV

Vous pouvez modifier ces classes selon vos besoins pour explorer PyO3.

### 4. Compiler et installer le module

```bash
maturin develop
```

Cette commande compile le code Rust et installe le module `piledger` dans votre
environnement virtuel Python actif.

## ✅ Tester le module

### Test de la classe Account

```bash
python tests/test_account.py
```

Ce test démontre :
- La création d'une instance `Account`
- La modification des attributs
- La gestion des types (erreur si on assigne une chaîne à un entier)

### Test de AccountReader

```bash
python tests/test_account_reader.py
```

Ce test démontre :
- La lecture d'un fichier CSV (`tests/comptes.csv`)
- L'itération sur les comptes avec une boucle `for`
- L'utilisation explicite de l'itérateur avec `__iter__()` et `next()`

## 📁 Structure du projet

```
├── Cargo.toml              # Configuration Rust/Cargo
├── pyproject.toml          # Configuration Python/Maturin
├── src/
│   └── lib.rs              # Code Rust avec PyO3
├── tests/
│   ├── comptes.csv         # Fichier CSV de test
│   ├── test_account.py     # Tests pour Account
│   └── test_account_reader.py  # Tests pour AccountReader
└── README.md               # Ce fichier
```


## 📚 Ressources

- [Documentation PyO3](https://pyo3.rs/)
- [Documentation Maturin](https://www.maturin.rs/)
