import facepaint from "facepaint";

// from Bootstrap SASS variables ($grid-breakpoints)
const breakpoints = [
    // <576 // xs aka. "default"
    576,    // sm
    768,    // md
    992,    // lg
    1200,   // xl
    1400,   // xxl
]
export const responsive = facepaint(breakpoints.map(pixels => `@media (min-width: ${pixels}px)`))
