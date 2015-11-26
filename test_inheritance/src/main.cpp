#include<stdio.h>
struct Node {
	float x;
	float y;
	void move_to(float x, float y) {
		this->x = x;
		this->y = y;
	}
	virtual void draw() const {
		printf("node: x = %f, y = %f\n", x, y);
	}
};

struct Sprite: public Node {
	int image_id;
	void set_image(int id) {
		image_id = id;
	}
	virtual void draw() const {
		printf("sprite: x = %f, y = %f, image = %d\n", x, y, image_id);
	}
};

int main() {
	Node node;
	node.move_to(10, 20);
	node.draw();
	Sprite sprite;
	sprite.move_to(10, 20);
	sprite.set_image(100);
	sprite.draw();
	return 0;
}
