pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }

    stages {
        stage('Install Tools') {
            steps {
                sh 'rustup component add rustfmt'
            }
        }

        stage('Format Check') {
            steps {
                sh 'cargo fmt -- --check'
            }
        }

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
    }
}
