export interface SheffieldYear {
  year: number
  name: string
  significance: string
  changes: string[]
  rougeActive: boolean
  isOfficial: boolean
}

export const SHEFFIELD_RULES_YEARS: SheffieldYear[] = [
  { year: 1858, name: 'Sheffield FC Founded', significance: 'The birth of Sheffield FC, one of the oldest football clubs still in existence.', changes: ['Club formation', 'Early informal rules'], rougeActive: false, isOfficial: false },
  { year: 1859, name: '1859', significance: 'Early years of Sheffield football development.', changes: ['Continued rule refinement'], rougeActive: false, isOfficial: false },
  { year: 1860, name: '1860', significance: 'Expansion of organized football in Sheffield. Only Sheffield FC and Hallam FC existed.', changes: ['Two clubs active', 'Early football development', 'Historical rivalry begins'], rougeActive: false, isOfficial: true },
  { year: 1861, name: '1861', significance: 'Pre-rouge era development.', changes: ['Rule standardization begins'], rougeActive: false, isOfficial: false },
  { year: 1862, name: 'Rouge Scoring Introduced', significance: 'The revolutionary rouge scoring system is introduced to Sheffield Rules.', changes: ['Rouge scoring begins', 'Behinds worth 1 point', 'Goals worth 2 points'], rougeActive: true, isOfficial: false },
  { year: 1863, name: '1863', significance: 'Rouge era established.', changes: ['Rouge system in effect'], rougeActive: true, isOfficial: false },
  { year: 1864, name: '1864', significance: 'Rouge scoring continues to develop.', changes: ['Refinements to rouge rules'], rougeActive: true, isOfficial: false },
  { year: 1865, name: '1865', significance: 'Peak of rouge era popularity.', changes: ['Rouge scoring well established'], rougeActive: true, isOfficial: false },
  { year: 1866, name: '1866', significance: 'Rouge system matures.', changes: ['Rouge rules refined'], rougeActive: true, isOfficial: false },
  { year: 1867, name: '1867 - Sheffield FA Founded', significance: 'Sheffield Football Association Established', changes: ['Sheffield FA founded', 'Final years of rouge scoring'], rougeActive: true, isOfficial: true },
  { year: 1868, name: 'Rouge Abolished', significance: 'The rouge system is discontinued as Sheffield Rules evolve.', changes: ['Rouge system removed', 'Goals-only scoring'], rougeActive: false, isOfficial: false },
  { year: 1869, name: '1869', significance: 'Post-rouge era begins.', changes: ['Goals-only scoring established'], rougeActive: false, isOfficial: false },
  { year: 1870, name: '1870', significance: 'Transition to modern scoring.', changes: ['Rule stabilization'], rougeActive: false, isOfficial: false },
  { year: 1871, name: '1871', significance: 'Post-rouge era consolidation.', changes: ['Rule refinement continues'], rougeActive: false, isOfficial: false },
  { year: 1872, name: '1872', significance: 'Convergence with FA Rules begins.', changes: ['Alignment with FA starting'], rougeActive: false, isOfficial: false },
  { year: 1873, name: '1873', significance: 'Moving toward FA Rules.', changes: ['Further FA alignment'], rougeActive: false, isOfficial: false },
  { year: 1874, name: '1874', significance: 'Sheffield Rules near FA standard.', changes: ['Nearly aligned with FA'], rougeActive: false, isOfficial: false },
  { year: 1875, name: 'Rules Stabilized', significance: 'Sheffield Rules reach a stable state near convergence with FA Rules.', changes: ['Rule standardization', 'Field dimensions finalized'], rougeActive: false, isOfficial: false },
  { year: 1876, name: '1876', significance: 'Final year before merger.', changes: ['Preparation for merger'], rougeActive: false, isOfficial: false },
  { year: 1877, name: 'Sheffield FA Merges', significance: 'Sheffield Rules formally merge with FA Rules, ending the Sheffield football era.', changes: ['Rules merger with FA', 'Adoption of standard football rules'], rougeActive: false, isOfficial: false },
]
