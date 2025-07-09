pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }

    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }

        stage('Lint') {
            steps {
                sh 'cargo fmt -- --check'
                sh 'cargo clippy -- -D warnings'
            }
        }
    }
}
