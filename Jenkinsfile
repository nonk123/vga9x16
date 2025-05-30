pipeline {
    agent any

    stages {
        stage('Build image') {
            steps {
                sh 'docker build -t vga9x16 .'
            }
        }

        stage('Deploy') {
            steps {
                sh 'docker container rm --force vga9x16 || true'
                sh 'docker run -d --name vga9x16 --network caddy --restart always vga9x16'
            }
        }
    }
}
