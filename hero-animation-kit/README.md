# Hero Animation Kit — Voice-to-Text Infinite Loop

Composant React autonome qui affiche une animation SVG de voix se transformant en texte, en boucle infinie.

## Contenu du kit

| Fichier | Description |
|---|---|
| `hero-graphic.tsx` | Composant React principal. Génère un SVG avec des ondes sinusoïdales animées (input vocal), un texte défilant le long d'une courbe de Bézier (output texte), des particules spark, et une icône centrale. Toutes les animations utilisent SMIL natif (`<animate>`) avec `repeatCount="indefinite"`. |
| `hero.module.css` | CSS Module avec les styles du conteneur, des ondes, du texte, des sparks et de l'icône. Inclut le support dark mode via `prefers-color-scheme`. |
| `app-logo.svg` | Logo SVG affiché au centre de l'animation (point de convergence voix → texte). |

## Prérequis

- React 18+ (seul `useMemo` est utilisé)
- Support CSS Modules dans ton bundler (Vite, Next.js, CRA…)
- **Aucune dépendance npm externe**

## Installation

1. Copie les 3 fichiers dans ton projet, par exemple dans `src/components/hero/`.

2. Place `app-logo.svg` dans ton dossier `public/` (ou modifie le `href` dans le composant).

3. Définis ces CSS variables dans ton CSS global :

```css
:root {
  --text-strong: #12151c;
  --text-muted: #808080;
}
@media (prefers-color-scheme: dark) {
  :root {
    --text-strong: #ffffff;
    --text-muted: #8c8c8c;
  }
}
```

4. Importe et utilise le composant :

```tsx
import { HeroGraphic } from "./components/hero/hero-graphic";

function App() {
  return (
    <div style={{ position: "relative", minHeight: 400 }}>
      <HeroGraphic />
    </div>
  );
}
```

## Comment ça marche

L'animation se décompose en 3 couches :

1. **Ondes sinusoïdales (input vocal)** — 3 ondes avec des fréquences et amplitudes différentes suivent une courbe de Bézier du haut vers l'icône centrale. Chaque onde a 20 frames pré-calculées pour un cycle fluide, animées en SMIL.

2. **Texte défilant (output texte)** — Un texte répété 10× défile le long d'une seconde courbe de Bézier qui part de l'icône vers le bas-droite. L'animation `startOffset` boucle sur 15.5 secondes.

3. **Particules spark** — 6 cercles SVG qui éclatent depuis l'icône avec des animations de position, opacité et taille.

## Personnalisation

- **Changer le logo** : remplace `app-logo.svg` ou modifie le `href` dans `hero-graphic.tsx`.
- **Changer le texte** : modifie la constante `BASE_TEXT` dans `hero-graphic.tsx`.
- **Ajuster les ondes** : modifie `WAVE_CONFIGS` (fréquence, amplitude, durée, opacité).
- **Ajuster les couleurs** : modifie les classes dans `hero.module.css` (`.wavePath`, `.pathText`, `.spark`).
- **Modifier les courbes** : ajuste les objets `inputCurve` et `outputCurve` (points de Bézier) dans le `useMemo`.
