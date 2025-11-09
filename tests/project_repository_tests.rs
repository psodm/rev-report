mod common;

use common::*;
use rev_report::db::repositories::project_repository::{ProjectRepository, ProjectRepositoryTrait};

#[test]
fn test_project_repository_new() {
    let repo = ProjectRepository::new();
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_project_repository_insert() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);

    let projects = repo.find_all();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_id, "PROJ-001");
}

#[test]
fn test_project_repository_insert_multiple() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    let project3 = create_test_project("PROJ-003", "Test Project 3");

    repo.insert(project1);
    repo.insert(project2);
    repo.insert(project3);

    let projects = repo.find_all();
    assert_eq!(projects.len(), 3);
}

#[test]
fn test_project_repository_find_by_id_exists() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);

    let found = repo.find_by_id("PROJ-001");
    assert!(found.is_some());
    let found_project = found.unwrap();
    assert_eq!(found_project.project_id, "PROJ-001");
    assert_eq!(found_project.project_name, "Test Project 1");
}

#[test]
fn test_project_repository_find_by_id_not_exists() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);

    let found = repo.find_by_id("PROJ-999");
    assert!(found.is_none());
}

#[test]
fn test_project_repository_insert_updates_existing() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project1);

    // Insert a new project with the same ID but different name
    let mut project2 = create_test_project("PROJ-001", "Updated Project Name");
    project2.end_customer_name = "Updated Customer".to_string();
    repo.insert(project2);

    // Should only have one project, and it should be the updated one
    let projects = repo.find_all();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_name, "Updated Project Name");
    assert_eq!(projects[0].end_customer_name, "Updated Customer");
}

#[test]
fn test_project_repository_find_all_empty() {
    let repo = ProjectRepository::new();
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_project_repository_delete_by_id_exists() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    repo.insert(project1);
    repo.insert(project2);

    // Verify both projects exist
    let projects = repo.find_all();
    assert_eq!(projects.len(), 2);

    // Delete one project
    let deleted = repo.delete_by_id("PROJ-001");
    assert!(deleted);

    // Verify it was deleted
    let projects_after = repo.find_all();
    assert_eq!(projects_after.len(), 1);
    assert_eq!(projects_after[0].project_id, "PROJ-002");

    // Verify it can't be found by ID
    let found = repo.find_by_id("PROJ-001");
    assert!(found.is_none());
}

#[test]
fn test_project_repository_delete_by_id_not_exists() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);

    // Try to delete a project that doesn't exist
    let deleted = repo.delete_by_id("PROJ-999");
    assert!(!deleted);

    // Verify original project still exists
    let projects = repo.find_all();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_id, "PROJ-001");
}

#[test]
fn test_project_repository_delete_by_id_empty_repository() {
    let repo = ProjectRepository::new();

    // Try to delete from empty repository
    let deleted = repo.delete_by_id("PROJ-001");
    assert!(!deleted);

    // Verify repository is still empty
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_project_repository_delete_all_projects() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    let project3 = create_test_project("PROJ-003", "Test Project 3");

    repo.insert(project1);
    repo.insert(project2);
    repo.insert(project3);

    // Delete all projects
    assert!(repo.delete_by_id("PROJ-001"));
    assert!(repo.delete_by_id("PROJ-002"));
    assert!(repo.delete_by_id("PROJ-003"));

    // Verify repository is empty
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}
