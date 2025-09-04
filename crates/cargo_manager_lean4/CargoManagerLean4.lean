import Lean

namespace CargoManagerLean4

-- Represents a Git repository
structure GitRepo where
  url : String
  commit : String
  path : System.FilePath
  deriving Repr, DecidableEq

-- Represents a dependency between two Git repositories
structure GitDependency where
  source : GitRepo
  target : GitRepo
  deriving Repr, DecidableEq

-- A collection of Git repositories, forming a "topological space"
-- For now, just a list, but conceptually it's a set of points.
structure GitRepoSpace where
  repos : List GitRepo
  deriving Repr

-- A collection of dependencies, forming a "lattice" structure
-- For now, just a list, but conceptually it's a set of edges.
structure GitDependencyLattice where
  dependencies : List GitDependency
  deriving Repr

-- The main state of our Cargo Manager, including the topological model
structure CargoManagerModel where
  repoSpace : GitRepoSpace
  dependencyLattice : GitDependencyLattice
  -- The root of our bootstrap, e.g., GNU Mes
  bootstrapRoot : GitRepo
  deriving Repr

end CargoManagerLean4
