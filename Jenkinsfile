pipeline {
    agent any

    stages {
        stage('Clone') {
            steps {
                git 'https://github.com/RaulIQ/system-monitor.git'
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
