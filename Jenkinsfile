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
                sh 'cargo install cargo-deb || true'
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
        stage('Build .deb') {
            steps {
                sh 'cargo deb --no-build '
            }
        }
    }
}
