use crate::lattice_point_derive::{LatticePoint, impl_lattice_point};

// Implement LatticePoint for all GitHub ecosystem structs - each becomes a URL/resource
impl_lattice_point!(GitHubRepository);
impl_lattice_point!(StarRelation);
impl_lattice_point!(ForkRelation);
impl_lattice_point!(ContributorRelation);
impl_lattice_point!(IssueNode);
impl_lattice_point!(PullRequestNode);
impl_lattice_point!(LanguageStats);
impl_lattice_point!(TopicTag);
impl_lattice_point!(LicenseType);
impl_lattice_point!(DependencyEdge);

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub enum GitHubEcosystem {
    Repository(GitHubRepository),
    Star(StarRelation),
    Fork(ForkRelation),
    Contributor(ContributorRelation),
    Issue(IssueNode),
    PullRequest(PullRequestNode),
    Language(LanguageStats),
    Topic(TopicTag),
    License(LicenseType),
    Dependency(DependencyEdge),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GitHubRepository {
    pub name: String,
    pub owner: String,
    pub url: String,
    pub stars: u32,
    pub forks: u32,
    pub last_commit_sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct StarRelation {
    pub user: String,
    pub repo: String,
    pub starred_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct ForkRelation {
    pub parent_repo: String,
    pub fork_repo: String,
    pub forked_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct ContributorRelation {
    pub user: String,
    pub repo: String,
    pub contributions: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct IssueNode {
    pub repo: String,
    pub number: u32,
    pub title: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct PullRequestNode {
    pub repo: String,
    pub number: u32,
    pub title: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageStats {
    pub repo: String,
    pub language: String,
    pub bytes: u64,
    pub percentage: u32, // Changed from f32 to u32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct TopicTag {
    pub repo: String,
    pub topic: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct LicenseType {
    pub repo: String,
    pub license: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct DependencyEdge {
    pub from_repo: String,
    pub to_repo: String,
    pub dependency_type: String,
}

impl GitHubEcosystem {
    pub fn from_10k_repos(repos: Vec<GitHubRepository>) -> Vec<Self> {
        repos.into_iter().map(Self::Repository).collect()
    }
    
    pub fn harvest_stars(&self) -> Vec<StarRelation> {
        match self {
            Self::Repository(repo) => {
                // Extract star relations from repo metadata
                vec![]
            }
            _ => vec![]
        }
    }
}
