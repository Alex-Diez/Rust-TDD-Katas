package game;

public class Game {

    private int score = 0;
    private int previousRoll = 0;
    private int roll = 0;
    private int bonus = 1;

    public void rollBall(int point) {
        roll++;
        score += bonus * point;
        if (roll % 2 == 0 && previousRoll + point == 10) {
            previousRoll = 0;
            bonus = 2;
        } else {
            previousRoll = point;
            bonus = 1;
        }
    }

    public int score() {
        return score;
    }
}
